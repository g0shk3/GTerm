use anyhow::{anyhow, Result};
use ssh2::{Channel, Session};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::{mpsc, Mutex, watch};

#[derive(Clone)]
pub struct SshConnection {
    session_id: String,
    session: Arc<Mutex<Option<Session>>>,
    channel: Arc<Mutex<Option<Channel>>>,
    tcp_stream: Arc<Mutex<Option<TcpStream>>>,
    shutdown_tx: Arc<Mutex<Option<watch::Sender<bool>>>>,
    input_tx: Arc<Mutex<Option<mpsc::UnboundedSender<Vec<u8>>>>>,
}

impl SshConnection {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            session: Arc::new(Mutex::new(None)),
            channel: Arc::new(Mutex::new(None)),
            tcp_stream: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(Mutex::new(None)),
            input_tx: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(
        &mut self,
        host: String,
        port: u16,
        username: String,
        private_key_path: String,
        passphrase: Option<String>,
        app_handle: tauri::AppHandle,
    ) -> Result<()> {
        let addr = format!("{}:{}", host, port);
        let tcp = TcpStream::connect(&addr)?;
        tcp.set_nodelay(true)?;

        let mut session = Session::new()?;
        session.set_tcp_stream(tcp.try_clone()?);
        session.handshake()?;

        // Authenticate with private key (in blocking mode)
        let key_path = Path::new(&private_key_path);
        if let Some(pass) = passphrase.as_ref() {
            session.userauth_pubkey_file(&username, None, key_path, Some(pass))?;
        } else {
            session.userauth_pubkey_file(&username, None, key_path, None)?;
        }

        if !session.authenticated() {
            return Err(anyhow!("Authentication failed"));
        }

        // Request PTY and start shell
        let mut channel = session.channel_session()?;
        channel.request_pty("xterm-256color", None, Some((80, 24, 0, 0)))?;
        channel.shell()?;
        channel.handle_extended_data(ssh2::ExtendedData::Merge)?;

        // NOW set session to non-blocking mode for I/O operations
        session.set_blocking(false);

        // Store session, tcp stream and channel
        *self.session.lock().await = Some(session);
        *self.tcp_stream.lock().await = Some(tcp);
        *self.channel.lock().await = Some(channel);

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);

        // Create input queue for non-blocking writes
        let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        *self.input_tx.lock().await = Some(input_tx);

        // Start UNIFIED I/O task that handles both reading and writing
        // This prevents lock contention and SSH protocol corruption
        let session_id = self.session_id.clone();
        let channel_arc = Arc::clone(&self.channel);

        tokio::task::spawn_blocking(move || {
            let mut read_buffer = [0u8; 8192];
            let mut write_buffer: Option<Vec<u8>> = None;
            let mut write_pos = 0;

            loop {
                // Check for shutdown
                if *shutdown_rx.borrow() {
                    break;
                }

                // Get channel lock once for this iteration
                let mut channel_guard = match channel_arc.try_lock() {
                    Ok(guard) => guard,
                    Err(_) => {
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        continue;
                    }
                };

                if let Some(ref mut channel) = *channel_guard {
                    // FIRST: Try to write pending data (if any)
                    if let Some(ref data) = write_buffer {
                        if write_pos < data.len() {
                            match channel.write(&data[write_pos..]) {
                                Ok(n) => {
                                    write_pos += n;
                                    if write_pos >= data.len() {
                                        // Finished writing this buffer
                                        write_buffer = None;
                                        write_pos = 0;
                                        let _ = channel.flush();
                                    }
                                }
                                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // Can't write now, will try again next iteration
                                }
                                Err(_) => {
                                    // Write error, discard buffer
                                    write_buffer = None;
                                    write_pos = 0;
                                }
                            }
                        }
                    } else {
                        // No pending write, check if there's new data to write
                        if let Ok(data) = input_rx.try_recv() {
                            write_buffer = Some(data);
                            write_pos = 0;
                        }
                    }

                    // SECOND: Try to read data
                    match channel.read(&mut read_buffer) {
                        Ok(0) => {
                            // EOF - connection closed
                            let _ = app_handle.emit(&format!("ssh-closed:{}", session_id), ());
                            break;
                        }
                        Ok(n) => {
                            // Successfully read data
                            let data = String::from_utf8_lossy(&read_buffer[..n]).to_string();
                            let _ = app_handle.emit(&format!("ssh-output:{}", session_id), data);
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // No data available
                        }
                        Err(_) => {
                            // Read error - might be transient, continue
                        }
                    }
                } else {
                    // No channel
                    break;
                }

                // Release lock
                drop(channel_guard);

                // Small sleep to prevent busy-waiting
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        });

        Ok(())
    }

    pub async fn send_input(&self, data: String) -> Result<()> {
        // Simply queue the input data - the writer task will handle it
        let input_tx = self.input_tx.lock().await;
        if let Some(ref tx) = *input_tx {
            tx.send(data.into_bytes())
                .map_err(|_| anyhow!("Failed to queue input - channel closed"))?;
            Ok(())
        } else {
            Err(anyhow!("No active input channel"))
        }
    }

    pub async fn resize(&self, cols: u32, rows: u32) -> Result<()> {
        let mut channel_guard = self.channel.lock().await;
        if let Some(ref mut channel) = *channel_guard {
            channel.request_pty_size(cols, rows, Some(0), Some(0))?;
            Ok(())
        } else {
            Err(anyhow!("No active channel"))
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        // Signal shutdown to reader and writer tasks
        if let Some(ref shutdown_tx) = *self.shutdown_tx.lock().await {
            let _ = shutdown_tx.send(true);
        }

        // Close input channel to stop writer task
        *self.input_tx.lock().await = None;

        // Small delay to allow tasks to exit gracefully
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        // Close channel
        if let Some(ref mut channel) = *self.channel.lock().await {
            let _ = channel.close();
            let _ = channel.wait_close();
        }

        // Disconnect session
        if let Some(ref mut session) = *self.session.lock().await {
            let _ = session.disconnect(None, "Client disconnecting", None);
        }

        // Clear all resources
        *self.channel.lock().await = None;
        *self.session.lock().await = None;
        *self.tcp_stream.lock().await = None;
        *self.shutdown_tx.lock().await = None;

        Ok(())
    }

    pub async fn get_session(&self) -> Arc<Mutex<Option<Session>>> {
        self.session.clone()
    }
}
