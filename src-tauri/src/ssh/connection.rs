use anyhow::{anyhow, Result};
use ssh2::{Channel, Session};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::{Mutex, watch};

#[derive(Clone)]
pub struct SshConnection {
    session_id: String,
    session: Arc<Mutex<Option<Session>>>,
    write_channel: Arc<Mutex<Option<Channel>>>,
    tcp_stream: Arc<Mutex<Option<TcpStream>>>,
    shutdown_tx: Arc<Mutex<Option<watch::Sender<bool>>>>,
}

impl SshConnection {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            session: Arc::new(Mutex::new(None)),
            write_channel: Arc::new(Mutex::new(None)),
            tcp_stream: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(Mutex::new(None)),
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

        // Authenticate with private key
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

        // Clone channel for reading
        let read_channel = channel.clone();

        // Store session and write channel
        *self.session.lock().await = Some(session);
        *self.tcp_stream.lock().await = Some(tcp);
        *self.write_channel.lock().await = Some(channel);

        // Create shutdown channel
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);

        // Start output reader in background with separate read channel
        let session_id = self.session_id.clone();
        tokio::task::spawn_blocking(move || {
            let mut buffer = [0u8; 8192];
            let mut read_chan = read_channel;

            loop {
                // Check for shutdown signal
                if *shutdown_rx.borrow() {
                    break;
                }

                // Set read timeout to allow periodic shutdown checks
                read_chan.set_read_timeout(Some(std::time::Duration::from_millis(100))).ok();

                match read_chan.read(&mut buffer) {
                    Ok(0) => {
                        // Connection closed
                        let _ = app_handle.emit(&format!("ssh-closed:{}", session_id), ());
                        break;
                    }
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                        let _ = app_handle.emit(&format!("ssh-output:{}", session_id), data);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock
                            || e.kind() == std::io::ErrorKind::TimedOut => {
                        // Timeout, check shutdown signal again
                        continue;
                    }
                    Err(_) => {
                        // Real error
                        let _ = app_handle.emit(&format!("ssh-error:{}", session_id), "Read error");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn send_input(&self, data: String) -> Result<()> {
        let mut channel_guard = self.write_channel.lock().await;
        if let Some(ref mut channel) = *channel_guard {
            channel.write_all(data.as_bytes())?;
            channel.flush()?;
            Ok(())
        } else {
            Err(anyhow!("No active channel"))
        }
    }

    pub async fn resize(&self, cols: u32, rows: u32) -> Result<()> {
        let mut channel_guard = self.write_channel.lock().await;
        if let Some(ref mut channel) = *channel_guard {
            channel.request_pty_size(cols, rows, Some(0), Some(0))?;
            Ok(())
        } else {
            Err(anyhow!("No active channel"))
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        // Signal shutdown to reader task
        if let Some(ref shutdown_tx) = *self.shutdown_tx.lock().await {
            let _ = shutdown_tx.send(true);
        }

        // Small delay to allow reader task to exit gracefully
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Close channel
        if let Some(ref mut channel) = *self.write_channel.lock().await {
            let _ = channel.close();
            let _ = channel.wait_close();
        }

        // Disconnect session
        if let Some(ref mut session) = *self.session.lock().await {
            let _ = session.disconnect(None, "Client disconnecting", None);
        }

        // Clear all resources
        *self.write_channel.lock().await = None;
        *self.session.lock().await = None;
        *self.tcp_stream.lock().await = None;
        *self.shutdown_tx.lock().await = None;

        Ok(())
    }

    pub async fn get_session(&self) -> Arc<Mutex<Option<Session>>> {
        self.session.clone()
    }
}

pub trait SshConnectionManager {
    async fn get_session(&self) -> Arc<Mutex<Option<Session>>>;
}

impl SshConnectionManager for SshConnection {
    async fn get_session(&self) -> Arc<Mutex<Option<Session>>> {
        self.session.clone()
    }
}
