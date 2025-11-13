use anyhow::{anyhow, Result};
use ssh2::{Channel, Session};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct SshConnection {
    session_id: String,
    session: Arc<Mutex<Option<Session>>>,
    channel: Arc<Mutex<Option<Channel>>>,
    tcp_stream: Arc<Mutex<Option<TcpStream>>>,
}

impl SshConnection {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            session: Arc::new(Mutex::new(None)),
            channel: Arc::new(Mutex::new(None)),
            tcp_stream: Arc::new(Mutex::new(None)),
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

        // Store session and channel
        *self.session.lock().await = Some(session);
        *self.tcp_stream.lock().await = Some(tcp);

        // Clone for background task
        let session_id = self.session_id.clone();
        let channel_clone = Arc::new(Mutex::new(channel));
        *self.channel.lock().await = Some(channel_clone.lock().await.try_clone()?);

        // Start output reader in background
        tokio::spawn(async move {
            let mut buffer = [0u8; 8192];
            loop {
                let mut channel_guard = channel_clone.lock().await;
                if let Some(ref mut chan) = *channel_guard {
                    match chan.read(&mut buffer) {
                        Ok(0) => {
                            // Connection closed
                            let _ = app_handle.emit(&format!("ssh-closed:{}", session_id), ());
                            break;
                        }
                        Ok(n) => {
                            let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                            let _ = app_handle.emit(&format!("ssh-output:{}", session_id), data);
                        }
                        Err(_) => {
                            // Error reading
                            let _ = app_handle.emit(&format!("ssh-error:{}", session_id), "Read error");
                            break;
                        }
                    }
                } else {
                    break;
                }
                drop(channel_guard);
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });

        Ok(())
    }

    pub async fn send_input(&self, data: String) -> Result<()> {
        let mut channel_guard = self.channel.lock().await;
        if let Some(ref mut channel) = *channel_guard {
            channel.write_all(data.as_bytes())?;
            channel.flush()?;
            Ok(())
        } else {
            Err(anyhow!("No active channel"))
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
        if let Some(ref mut channel) = *self.channel.lock().await {
            let _ = channel.close();
            let _ = channel.wait_close();
        }

        if let Some(ref mut session) = *self.session.lock().await {
            let _ = session.disconnect(None, "Client disconnecting", None);
        }

        *self.channel.lock().await = None;
        *self.session.lock().await = None;
        *self.tcp_stream.lock().await = None;

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
