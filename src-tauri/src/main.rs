// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ssh;

use ssh::{
    connection::{SshConnection, SshConnectionManager},
    keygen::{generate_ed25519_keypair, get_key_type},
    sftp::{list_directory, download_file, upload_file},
};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

pub struct AppState {
    connections: Arc<Mutex<HashMap<String, SshConnection>>>,
}

#[tauri::command]
async fn ssh_connect(
    session_id: String,
    host: String,
    port: u16,
    username: String,
    private_key_path: String,
    passphrase: Option<String>,
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut connection = SshConnection::new(session_id.clone());

    connection
        .connect(host, port, username, private_key_path, passphrase, app_handle)
        .await
        .map_err(|e| e.to_string())?;

    let mut connections = state.connections.lock().await;
    connections.insert(session_id.clone(), connection);

    Ok(session_id)
}

#[tauri::command]
async fn ssh_send_input(
    session_id: String,
    data: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let connections = state.connections.lock().await;

    if let Some(connection) = connections.get(&session_id) {
        connection.send_input(data).await.map_err(|e| e.to_string())
    } else {
        Err("Connection not found".to_string())
    }
}

#[tauri::command]
async fn ssh_disconnect(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut connections = state.connections.lock().await;

    if let Some(mut connection) = connections.remove(&session_id) {
        connection.disconnect().await.map_err(|e| e.to_string())
    } else {
        Err("Connection not found".to_string())
    }
}

#[tauri::command]
async fn ssh_resize(
    session_id: String,
    cols: u32,
    rows: u32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let connections = state.connections.lock().await;

    if let Some(connection) = connections.get(&session_id) {
        connection.resize(cols, rows).await.map_err(|e| e.to_string())
    } else {
        Err("Connection not found".to_string())
    }
}

#[tauri::command]
async fn get_home_dir() -> Result<String, String> {
    dirs::home_dir()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .ok_or_else(|| "Failed to get home directory".to_string())
}

#[tauri::command]
async fn get_private_key_type(path: String) -> Result<String, String> {
    get_key_type(&path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn generate_keypair(output_path: String) -> Result<String, String> {
    generate_ed25519_keypair(&output_path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn sftp_list_directory(
    session_id: String,
    path: String,
    state: State<'_, AppState>,
) -> Result<Vec<ssh::sftp::FileEntry>, String> {
    let connections = state.connections.lock().await;

    if let Some(connection) = connections.get(&session_id) {
        list_directory(connection, &path).await.map_err(|e| e.to_string())
    } else {
        Err("Connection not found".to_string())
    }
}

#[tauri::command]
async fn sftp_download(
    session_id: String,
    remote_path: String,
    local_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let connections = state.connections.lock().await;

    if let Some(connection) = connections.get(&session_id) {
        download_file(connection, &remote_path, &local_path).await.map_err(|e| e.to_string())
    } else {
        Err("Connection not found".to_string())
    }
}

#[tauri::command]
async fn sftp_upload(
    session_id: String,
    local_path: String,
    remote_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let connections = state.connections.lock().await;

    if let Some(connection) = connections.get(&session_id) {
        upload_file(connection, &local_path, &remote_path).await.map_err(|e| e.to_string())
    } else {
        Err("Connection not found".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            connections: Arc::new(Mutex::new(HashMap::new())),
        })
        .invoke_handler(tauri::generate_handler![
            ssh_connect,
            ssh_send_input,
            ssh_disconnect,
            ssh_resize,
            get_home_dir,
            get_private_key_type,
            generate_keypair,
            sftp_list_directory,
            sftp_download,
            sftp_upload,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
