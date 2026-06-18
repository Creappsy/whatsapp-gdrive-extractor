mod extractor;

use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use std::path::PathBuf;
use tauri::{Emitter, State};
use serde::{Serialize, Deserialize};
use base64::Engine;

const CONFIG_FILE: &str = "settings.json";

#[derive(Serialize)]
pub struct LocalConfig {
    pub gmail: String,
    pub android_id: String,
    pub has_oauth_token: bool,
    pub has_password: bool,
}

#[derive(Deserialize, Serialize)]
struct StoredConfig {
    gmail: String,
    android_id: String,
}

pub struct AppState {
    pub client: Mutex<Option<extractor::WaBackupClient>>,
}

// Helper to load settings.json
fn load_local_config_file() -> StoredConfig {
    if let Ok(mut file) = File::open(CONFIG_FILE) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(cfg) = serde_json::from_str::<StoredConfig>(&contents) {
                return cfg;
            }
        }
    }
    StoredConfig {
        gmail: "".to_string(),
        android_id: "".to_string(),
    }
}

// Helper to save settings.json
fn save_local_config_file(gmail: &str, android_id: &str) {
    let cfg = StoredConfig {
        gmail: gmail.to_string(),
        android_id: android_id.to_string(),
    };
    if let Ok(mut file) = File::create(CONFIG_FILE) {
        if let Ok(json) = serde_json::to_string_pretty(&cfg) {
            let _ = file.write_all(json.as_bytes());
        }
    }
}

#[tauri::command]
async fn get_config() -> Result<LocalConfig, String> {
    let stored = load_local_config_file();
    
    let mut has_token = false;
    let mut has_password = false;
    
    if !stored.gmail.is_empty() {
        // Check keyring for oauth token
        let token_service = format!("WhatsAppGDriveExtractor_token");
        if let Ok(entry) = keyring::Entry::new(&token_service, &stored.gmail) {
            if entry.get_password().is_ok() {
                has_token = true;
            }
        }
        
        // Check keyring for app password
        let pass_service = format!("WhatsAppGDriveExtractor_pass");
        if let Ok(entry) = keyring::Entry::new(&pass_service, &stored.gmail) {
            if entry.get_password().is_ok() {
                has_password = true;
            }
        }
    }
    
    Ok(LocalConfig {
        gmail: stored.gmail,
        android_id: stored.android_id,
        has_oauth_token: has_token,
        has_password,
    })
}

#[tauri::command]
async fn login(
    state: State<'_, AppState>,
    gmail: String,
    password: Option<String>,
    oauth_token: Option<String>,
    android_id: Option<String>,
) -> Result<String, String> {
    // Generate android ID if empty
    let aid = if let Some(ref id) = android_id {
        if id.trim().is_empty() {
            generate_random_android_id()
        } else {
            id.trim().to_string()
        }
    } else {
        generate_random_android_id()
    };
    
    let email = gmail.trim().to_lowercase();
    
    // Attempt login
    // First, check if inputs are placeholders (which means we load them from Keyring)
    let mut actual_token = oauth_token.clone();
    let mut actual_password = password.clone();
    
    if let Some(ref t) = oauth_token {
        if t == "••••••••••••" {
            let token_service = format!("WhatsAppGDriveExtractor_token");
            if let Ok(entry) = keyring::Entry::new(&token_service, &email) {
                if let Ok(stored_t) = entry.get_password() {
                    actual_token = Some(stored_t);
                }
            }
        }
    }
    
    if let Some(ref p) = password {
        if p == "••••••••••••" {
            let pass_service = format!("WhatsAppGDriveExtractor_pass");
            if let Ok(entry) = keyring::Entry::new(&pass_service, &email) {
                if let Ok(stored_p) = entry.get_password() {
                    actual_password = Some(stored_p);
                }
            }
        }
    }
    
    let client_res = extractor::WaBackupClient::new(
        &email,
        &aid,
        actual_password.as_deref(),
        actual_token.as_deref()
    ).await;
    
    match client_res {
        Ok(client) => {
            // Save to Keyring on success
            if let Some(ref t) = actual_token {
                if !t.trim().is_empty() && t != "••••••••••••" {
                    let token_service = format!("WhatsAppGDriveExtractor_token");
                    if let Ok(entry) = keyring::Entry::new(&token_service, &email) {
                        let _ = entry.set_password(t);
                    }
                }
            }
            if let Some(ref p) = actual_password {
                if !p.trim().is_empty() && p != "••••••••••••" {
                    let pass_service = format!("WhatsAppGDriveExtractor_pass");
                    if let Ok(entry) = keyring::Entry::new(&pass_service, &email) {
                        let _ = entry.set_password(p);
                    }
                }
            }
            
            // Save settings.json
            save_local_config_file(&email, &aid);
            
            // Set global state client
            let mut guard = state.client.lock().unwrap();
            *guard = Some(client);
            
            Ok("Login exitoso".to_string())
        }
        Err(e) => Err(format!("Fallo de inicio de sesión: {}", e))
    }
}

#[tauri::command]
async fn logout(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.client.lock().unwrap();
    *guard = None;
    Ok(())
}

#[tauri::command]
async fn list_backups(state: State<'_, AppState>) -> Result<Vec<extractor::BackupInfo>, String> {
    let client = {
        let guard = state.client.lock().unwrap();
        guard.clone()
    };
    if let Some(client) = client {
        match client.backups().await {
            Ok(backups) => {
                let infos = backups.iter().map(|b| extractor::get_backup_info(b)).collect();
                Ok(infos)
            }
            Err(e) => Err(format!("Error listando backups: {}", e))
        }
    } else {
        Err("No hay sesión activa".to_string())
    }
}

#[derive(Serialize, Clone)]
struct ProgressPayload {
    num_files: usize,
    total_files: usize,
    chunk_size: usize,
    done: bool,
    error: Option<String>,
}

#[tauri::command]
async fn download_backup(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    backup_id: String
) -> Result<String, String> {
    // 1. Get client
    let client = {
        let guard = state.client.lock().unwrap();
        match &*guard {
            Some(c) => c.gmail.clone(), // just check if session exists
            None => return Err("No hay sesión activa".to_string())
        }
    };
    
    // We clone the client reference by re-obtaining lock when needed, or we can borrow.
    // To download in the background without blocking Tauri main thread, we spawn a tokio task:
    let backup_name = format!("clients/wa/backups/{}", backup_id);
    let state_clone = state.client.lock().unwrap().clone();
    let client = match state_clone {
        Some(c) => std::sync::Arc::new(c),
        None => return Err("No hay sesión activa".to_string())
    };
    
    tokio::spawn(async move {
        // Fetch files list
        let files = match client.backup_files(&backup_name).await {
            Ok(f) => f,
            Err(e) => {
                let _ = app_handle.emit("download_progress", ProgressPayload {
                    num_files: 0,
                    total_files: 0,
                    chunk_size: 0,
                    done: false,
                    error: Some(format!("Error obteniendo archivos: {}", e)),
                });
                return;
            }
        };
        
        let total_files = files.len();
        let mut num_files = 0;
        let mut cksum_lines = Vec::new();
        
        // Ensure local folder exists
        let backup_dir = PathBuf::from(&backup_id);
        let _ = std::fs::create_dir_all(&backup_dir);
        
        for file in files {
            // Process filename relative to backup dir
            // Extract filename from "clients/wa/backups/<id>/files/..."
            // Usually, file["name"].split("/")[5:] gets us the Databases/Media layout
            let name_parts: Vec<&str> = file.name.split('/').collect();
            let relative_parts = if name_parts.len() > 5 {
                &name_parts[5..]
            } else {
                &name_parts[3..]
            };
            
            let mut file_path = backup_dir.clone();
            for part in relative_parts {
                file_path.push(part);
            }
            
            // Skip download if MD5 matches
            let md5_decoded = file.md5Hash.as_ref()
                .and_then(|h| base64::engine::general_purpose::STANDARD.decode(h).ok());
            let file_size = file.sizeBytes.as_ref()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
                
            let mut downloaded = false;
            if file_path.exists() {
                if let Ok(metadata) = std::fs::metadata(&file_path) {
                    if metadata.len() == file_size {
                        downloaded = true; // Simple check (size matches)
                    }
                }
            }
            
            if !downloaded {
                // Ensure parent dirs exist
                if let Some(parent) = file_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                
                // Fetch file bytes from Google Drive
                match client.fetch_file_bytes(&file.name).await {
                    Ok(bytes) => {
                        if std::fs::write(&file_path, &bytes).is_ok() {
                            num_files += 1;
                            let _ = app_handle.emit("download_progress", ProgressPayload {
                                num_files,
                                total_files,
                                chunk_size: bytes.len(),
                                done: false,
                                error: None,
                            });
                        }
                    }
                    Err(e) => {
                        let _ = app_handle.emit("download_progress", ProgressPayload {
                            num_files,
                            total_files,
                            chunk_size: 0,
                            done: false,
                            error: Some(format!("Error descargando archivo: {}", e)),
                        });
                        return;
                    }
                }
            } else {
                num_files += 1;
                let _ = app_handle.emit("download_progress", ProgressPayload {
                    num_files,
                    total_files,
                    chunk_size: 0,
                    done: false,
                    error: None,
                });
            }
            
            // Record checksums
            let md5_hex = md5_decoded.map(|d| hex::encode(d)).unwrap_or_else(|| "".to_string());
            let display_name = relative_parts.join("/");
            cksum_lines.push(format!("{} *{}\n", md5_hex, display_name));
        }
        
        // Write md5sum.txt
        let _ = std::fs::write("md5sum.txt", cksum_lines.join(""));
        
        // Emit done event
        let _ = app_handle.emit("download_progress", ProgressPayload {
            num_files: total_files,
            total_files,
            chunk_size: 0,
            done: true,
            error: None,
        });
    });
    
    Ok("Download started".to_string())
}

#[tauri::command]
async fn open_backup_folder(backup_id: String) -> Result<(), String> {
    let mut backup_path = std::env::current_dir().map_err(|e| e.to_string())?;
    backup_path.push(&backup_id);
    
    if !backup_path.exists() {
        return Err("La copia de seguridad no se ha descargado todavía localmente.".to_string());
    }
    
    // Open explorer window
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer").arg(backup_path).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(backup_path).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(backup_path).spawn();
    }
    
    Ok(())
}

#[tauri::command]
async fn decrypt_db(file_base64: String, filename: String, key_hex: String) -> Result<String, String> {
    let crypt_bytes = base64::engine::general_purpose::STANDARD.decode(&file_base64)
        .map_err(|e| format!("Base64 decoding failed: {}", e))?;
        
    let decrypted_bytes = extractor::decrypt_db_bytes(&key_hex, &crypt_bytes)
        .map_err(|e| format!("{}", e))?;
        
    // Save locally or return base64
    let out_filename = format!("{}.db", filename);
    let mut out_path = std::env::current_dir().map_err(|e| e.to_string())?;
    out_path.push(&out_filename);
    
    std::fs::write(&out_path, &decrypted_bytes)
        .map_err(|e| format!("Error guardando archivo: {}", e))?;
        
    // Return base64 of decrypted DB for browser download fallback
    let out_base64 = base64::engine::general_purpose::STANDARD.encode(&decrypted_bytes);
    Ok(out_base64)
}

#[derive(Serialize)]
struct UpdateInfo {
    current_version: String,
    latest_version: String,
    update_available: bool,
    url: String,
}

#[tauri::command]
async fn check_update() -> Result<UpdateInfo, String> {
    let current_version = "1.0.0".to_string();
    
    let client = reqwest::Client::builder()
        .user_agent("WhatsApp-GDrive-Extractor")
        .build()
        .map_err(|e| e.to_string())?;
        
    let url = "https://api.github.com/repos/daferferso/whatsapp-gdrive-extractor/releases/latest";
    match client.get(url).timeout(std::time::Duration::from_secs(5)).send().await {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                let latest_version = json.get("tag_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("1.0.0")
                    .trim_start_matches('v')
                    .to_string();
                let html_url = json.get("html_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("https://github.com/daferferso/whatsapp-gdrive-extractor/releases")
                    .to_string();
                    
                let cur_parts: Vec<u32> = current_version.split('.').filter_map(|x| x.parse().ok()).collect();
                let lat_parts: Vec<u32> = latest_version.split('.').filter_map(|x| x.parse().ok()).collect();
                
                let update_available = if cur_parts.len() == 3 && lat_parts.len() == 3 {
                    lat_parts > cur_parts
                } else {
                    false
                };
                
                return Ok(UpdateInfo {
                    current_version,
                    latest_version,
                    update_available,
                    url: html_url,
                });
            }
        }
        _ => {}
    }
    
    Ok(UpdateInfo {
        current_version: current_version.clone(),
        latest_version: current_version,
        update_available: false,
        url: "".to_string(),
    })
}

#[tauri::command]
async fn push_to_phone(backup_id: String) -> Result<Vec<String>, String> {
    let mut backup_path = std::env::current_dir().map_err(|e| e.to_string())?;
    backup_path.push(&backup_id);
    backup_path.push("files");
    
    if !backup_path.exists() {
        return Err("La copia de seguridad no se ha descargado localmente aún. Por favor, descárgala primero.".to_string());
    }
    
    let check_adb = std::process::Command::new("adb").arg("devices").output();
    let output = match check_adb {
        Ok(out) => out,
        Err(_) => return Err("El comando 'adb' no está disponible en la PC. Instala Android Platform Tools.".to_string())
    };
    
    let devices_str = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    for line in devices_str.lines() {
        if line.contains("device") && !line.contains("List of devices") {
            if let Some(d) = line.split_whitespace().next() {
                devices.push(d.to_string());
            }
        }
    }
    
    if devices.is_empty() {
        return Err("No se detectó ningún celular conectado. Asegúrate de conectarlo por cable y activar la 'Depuración USB'.".to_string());
    }
    
    let subdirs = ["Databases", "Backups"];
    let paths = [
        "/sdcard/WhatsApp",
        "/sdcard/Android/media/com.whatsapp/WhatsApp"
    ];
    
    let mut logs = Vec::new();
    
    for subdir in &subdirs {
        let local_subdir = backup_path.join(subdir);
        if local_subdir.exists() {
            for dest_base in &paths {
                let dest_subdir = format!("{}/{}", dest_base, subdir);
                
                let _ = std::process::Command::new("adb")
                    .args(&["shell", "mkdir", "-p", &dest_subdir])
                    .output();
                    
                let _ = std::process::Command::new("adb")
                    .args(&["push", local_subdir.to_str().unwrap(), dest_base])
                    .output();
                    
                logs.push(format!("Copiado: {} a {}", subdir, dest_base));
            }
        }
    }
    
    Ok(logs)
}

fn generate_random_android_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let num: u64 = rng.gen();
    format!("{:016x}", num)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(AppState {
        client: Mutex::new(None),
    })
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        get_config,
        login,
        logout,
        list_backups,
        download_backup,
        open_backup_folder,
        decrypt_db,
        check_update,
        push_to_phone
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
