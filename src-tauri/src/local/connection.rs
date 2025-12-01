use anyhow::{anyhow, Result};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::sync::{mpsc, watch, Mutex};
use std::env;
use tauri::Emitter;

#[derive(Clone)]
pub struct LocalConnection {
    session_id: String,
    pty_pair: Arc<Mutex<Option<Box<dyn portable_pty::MasterPty + Send>>>>,
    child: Arc<Mutex<Option<Box<dyn portable_pty::Child + Send + Sync>>>>,
    reader: Arc<Mutex<Option<Box<dyn Read + Send>>>>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    shutdown_tx: Arc<Mutex<Option<watch::Sender<bool>>>>,
    input_tx: Arc<Mutex<Option<mpsc::UnboundedSender<Vec<u8>>>>>,
}

impl LocalConnection {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            pty_pair: Arc::new(Mutex::new(None)),
            child: Arc::new(Mutex::new(None)),
            reader: Arc::new(Mutex::new(None)),
            writer: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(Mutex::new(None)),
            input_tx: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn spawn(
        &mut self,
        shell: Option<String>,
        cwd: Option<String>,
        app_handle: tauri::AppHandle,
    ) -> Result<()> {
        // Get shell to use
        let shell_path = shell.unwrap_or_else(|| {
            env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string())
        });

        // Get working directory
        let working_dir = cwd.unwrap_or_else(|| {
            env::var("HOME").unwrap_or_else(|_| "/".to_string())
        });

        // Create PTY system
        let pty_system = NativePtySystem::default();

        // Create PTY with initial size
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| anyhow!("Failed to create PTY: {}", e))?;

        // Create command
        let mut cmd = CommandBuilder::new(&shell_path);
        cmd.cwd(&working_dir);

        // Set environment variables for better shell experience
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");

        // Set UTF-8 locale for proper Unicode/Cyrillic character support
        cmd.env("LANG", "en_US.UTF-8");
        cmd.env("LC_ALL", "en_US.UTF-8");

        // Spawn child process
        let child = pty_pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| anyhow!("Failed to spawn shell: {}", e))?;

        // Get reader and writer from master PTY
        let reader = pty_pair.master.try_clone_reader()
            .map_err(|e| anyhow!("Failed to clone reader: {}", e))?;

        let writer = pty_pair.master.take_writer()
            .map_err(|e| anyhow!("Failed to get writer: {}", e))?;

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        // Create input channel
        let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Vec<u8>>();

        // Store handles
        *self.pty_pair.lock().await = Some(pty_pair.master);
        *self.child.lock().await = Some(child);
        *self.reader.lock().await = Some(reader);
        *self.writer.lock().await = Some(writer);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);
        *self.input_tx.lock().await = Some(input_tx);

        // Clone what we need for the I/O tasks
        let session_id_clone = self.session_id.clone();
        let mut writer_clone = self.writer.lock().await.take().ok_or_else(|| anyhow!("Failed to take writer"))?;
        let mut reader_clone = self.reader.lock().await.take().ok_or_else(|| anyhow!("Failed to take reader"))?;
        let child_clone = self.child.clone();
        let shutdown_rx_clone = shutdown_rx.clone();

        // Writer task
        tokio::spawn(async move {
            while let Some(data) = input_rx.recv().await {
                if let Err(_e) = writer_clone.write_all(&data) {
                    break;
                }
                if let Err(_e) = writer_clone.flush() {
                    break;
                }
            }
        });

        // Reader task with adaptive sleep to reduce idle wake ups
        let app_handle_clone = app_handle.clone();
        tokio::task::spawn_blocking(move || {
            let mut buffer = [0u8; 8192];
            let mut idle_count = 0u32;

            loop {
                if *shutdown_rx_clone.borrow() {
                    break;
                }

                match reader_clone.read(&mut buffer) {
                    Ok(0) => { // EOF
                        let _ = app_handle_clone.emit(&format!("terminal-closed:{}", session_id_clone), "Shell exited");
                        break;
                    },
                    Ok(n) => {
                        let data = &buffer[..n];
                        let _ = app_handle_clone.emit(&format!("terminal-output:{}", session_id_clone), data);
                        idle_count = 0; // Reset on activity
                    },
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Adaptive sleep: fast when active, slower when idle
                        idle_count += 1;
                        let sleep_duration = if idle_count < 10 {
                            std::time::Duration::from_micros(100)  // Active: 10k checks/sec
                        } else if idle_count < 100 {
                            std::time::Duration::from_millis(1)    // Idle: 1k checks/sec
                        } else {
                            std::time::Duration::from_millis(5)    // Very idle: 200 checks/sec
                        };
                        std::thread::sleep(sleep_duration);
                        continue;
                    },
                    Err(_e) => {
                        // Optional: log read error
                        let _ = app_handle_clone.emit(&format!("terminal-error:{}", session_id_clone), "Read error");
                        break;
                    }
                }
            }

            // Clean up child process on exit
            if let Ok(mut child_guard) = child_clone.try_lock() {
                if let Some(ref mut child) = *child_guard {
                    let _ = child.kill();
                }
            }
        });

        Ok(())
    }

    pub async fn send_input(&self, data: String) -> Result<()> {
        let input_tx = self.input_tx.lock().await;
        if let Some(ref tx) = *input_tx {
            tx.send(data.into_bytes())
                .map_err(|_| anyhow!("Failed to queue input"))?;
            Ok(())
        } else {
            Err(anyhow!("No active input channel"))
        }
    }

    pub async fn resize(&self, cols: u32, rows: u32) -> Result<()> {
        let mut pty_guard = self.pty_pair.lock().await;
        if let Some(ref mut pty) = *pty_guard {
            pty.resize(PtySize {
                rows: rows as u16,
                cols: cols as u16,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| anyhow!("Failed to resize PTY: {}", e))
        } else {
            Err(anyhow!("No active PTY"))
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        // Signal shutdown
        if let Some(ref tx) = *self.shutdown_tx.lock().await {
            let _ = tx.send(true);
        }

        // Kill child process
        let mut child_guard = self.child.lock().await;
        if let Some(ref mut child) = *child_guard {
            child.kill().map_err(|e| anyhow!("Failed to kill child: {}", e))?;
        }

        // Clear all handles
        *child_guard = None;
        *self.pty_pair.lock().await = None;
        *self.reader.lock().await = None;
        *self.writer.lock().await = None;
        *self.shutdown_tx.lock().await = None;
        *self.input_tx.lock().await = None;

        Ok(())
    }
}
