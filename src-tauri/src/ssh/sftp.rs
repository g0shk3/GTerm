use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::Path;

use super::connection::SshConnectionManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: i64,
    pub permissions: u32,
}

pub async fn list_directory<T: SshConnectionManager>(
    connection: &T,
    path: &str,
) -> Result<Vec<FileEntry>> {
    let session_arc = connection.get_session().await;
    let session_guard = session_arc.lock().await;

    let session = session_guard
        .as_ref()
        .ok_or_else(|| anyhow!("No active session"))?;

    let sftp = session.sftp()?;
    let dir_path = Path::new(path);

    let mut entries = Vec::new();

    for (path_buf, stat) in sftp.readdir(dir_path)? {
        let name = path_buf
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let full_path = path_buf.to_string_lossy().to_string();

        entries.push(FileEntry {
            name,
            path: full_path,
            is_dir: stat.is_dir(),
            size: stat.size.unwrap_or(0),
            modified: stat.mtime.unwrap_or(0) as i64,
            permissions: stat.perm.unwrap_or(0),
        });
    }

    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(entries)
}

pub async fn download_file<T: SshConnectionManager>(
    connection: &T,
    remote_path: &str,
    local_path: &str,
) -> Result<()> {
    let session_arc = connection.get_session().await;
    let session_guard = session_arc.lock().await;

    let session = session_guard
        .as_ref()
        .ok_or_else(|| anyhow!("No active session"))?;

    let sftp = session.sftp()?;

    let mut remote_file = sftp.open(Path::new(remote_path))?;
    let mut local_file = std::fs::File::create(local_path)?;

    let mut buffer = vec![0; 8192];
    loop {
        let n = remote_file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        local_file.write_all(&buffer[..n])?;
    }

    Ok(())
}

pub async fn upload_file<T: SshConnectionManager>(
    connection: &T,
    local_path: &str,
    remote_path: &str,
) -> Result<()> {
    let session_arc = connection.get_session().await;
    let session_guard = session_arc.lock().await;

    let session = session_guard
        .as_ref()
        .ok_or_else(|| anyhow!("No active session"))?;

    let sftp = session.sftp()?;

    let mut local_file = std::fs::File::open(local_path)?;
    let mut remote_file = sftp.create(Path::new(remote_path))?;

    let mut buffer = vec![0; 8192];
    loop {
        let n = local_file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        remote_file.write_all(&buffer[..n])?;
    }

    Ok(())
}
