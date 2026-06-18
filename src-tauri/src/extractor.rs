use std::collections::HashMap;
use std::path::{Path, PathBuf};
use aes_gcm::{AesGcm, aes::Aes256, aead::{Aead, KeyInit, consts::U16}};
use base64::Engine;
use rsa::{RsaPublicKey, Oaep, BigUint};
use sha1::Sha1;
use serde::{Serialize, Deserialize};

const B64_KEY_7_3_29: &[u8] = b"AAAAgMom/1a/v0lblO2Ubrt60J2gcuXSljGFQXgcyZWveWLEwo6prwgi3iJIZdodyhKZQrNWp5nKJ3srRXcUW+F1BD3baEVGcmEgqaLZUNBjm057pKRI16kB0YppeGx5qIQ5QjKzsR8ETQbKLNWgRY0QRNVz34kMJR3P/LgHax/6rmf5AAAAAwEAAQ==";
const KEY_HASH_PREFIX: &[u8] = &[0x57, 0x1b, 0xe0, 0xa4];
const AUTH_URL: &str = "https://android.clients.google.com/auth";
const USER_AGENT: &str = "GoogleAuth/1.4";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupItem {
    pub name: String,
    pub updateTime: String,
    pub metadata: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupInfo {
    pub id: String,
    pub backupSize: String,
    pub backupSizeHuman: String,
    pub uploadTime: String,
    pub whatsappVersion: String,
    pub messages: String,
    pub mediaFiles: String,
    pub photos: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileItem {
    pub name: String,
    pub sizeBytes: Option<String>,
    pub md5Hash: Option<String>,
}

#[derive(Clone)]
pub struct WaBackupClient {
    pub gmail: String,
    pub android_id: String,
    pub auth_token: String,
    client: reqwest::Client,
}

impl WaBackupClient {
    pub async fn new(
        gmail: &str,
        android_id: &str,
        password: Option<&str>,
        oauth_token: Option<&str>
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::builder()
            .http1_only()
            .build()?;

        let mut master_token = String::new();

        if let Some(token) = oauth_token {
            if !token.trim().is_empty() {
                // Exchange web token for master token
                let mut data = HashMap::new();
                data.insert("accountType", "HOSTED_OR_GOOGLE".to_string());
                data.insert("Email", gmail.trim().to_lowercase());
                data.insert("has_permission", "1".to_string());
                data.insert("add_account", "1".to_string());
                data.insert("ACCESS_TOKEN", "1".to_string());
                data.insert("Token", token.trim().to_string());
                data.insert("service", "ac2dm".to_string());
                data.insert("source", "android".to_string());
                data.insert("androidId", android_id.trim().to_string());
                data.insert("device_country", "us".to_string());
                data.insert("operatorCountry", "us".to_string());
                data.insert("lang", "en".to_string());
                data.insert("sdk_version", "17".to_string());
                data.insert("google_play_services_version", "240913000".to_string());
                data.insert("client_sig", "38918a453d07199354f8b19af05ec6562ced5788".to_string());
                data.insert("callerSig", "38918a453d07199354f8b19af05ec6562ced5788".to_string());
                data.insert("droidguard_results", "dummy123".to_string());

                let res_map = perform_auth_request(&client, &data).await?;
                if let Some(t) = res_map.get("Token") {
                    master_token = t.clone();
                } else {
                    return Err(format!("Auth failed (token exchange): {:?}", res_map).into());
                }
            }
        }

        if master_token.is_empty() {
            if let Some(pass) = password {
                let sanitized_pass = pass.trim().replace(" ", "");
                let encrypted_pass = encrypt_password(gmail, &sanitized_pass)?;

                let mut data = HashMap::new();
                data.insert("accountType", "HOSTED_OR_GOOGLE".to_string());
                data.insert("Email", gmail.trim().to_lowercase());
                data.insert("has_permission", "1".to_string());
                data.insert("add_account", "1".to_string());
                data.insert("EncryptedPasswd", encrypted_pass);
                data.insert("service", "ac2dm".to_string());
                data.insert("source", "android".to_string());
                data.insert("androidId", android_id.trim().to_string());
                data.insert("device_country", "us".to_string());
                data.insert("operatorCountry", "us".to_string());
                data.insert("lang", "en".to_string());
                data.insert("sdk_version", "17".to_string());
                data.insert("google_play_services_version", "240913000".to_string());
                data.insert("client_sig", "38918a453d07199354f8b19af05ec6562ced5788".to_string());
                data.insert("callerSig", "38918a453d07199354f8b19af05ec6562ced5788".to_string());
                data.insert("droidguard_results", "dummy123".to_string());

                let res_map = perform_auth_request(&client, &data).await?;
                if let Some(t) = res_map.get("Token") {
                    master_token = t.clone();
                } else {
                    return Err(format!("Auth failed (master login): {:?}", res_map).into());
                }
            } else {
                return Err("No password or oauth_token provided".into());
            }
        }

        // Perform OAuth request to get Drive scope token
        let mut data = HashMap::new();
        data.insert("accountType", "HOSTED_OR_GOOGLE".to_string());
        data.insert("Email", gmail.trim().to_lowercase());
        data.insert("has_permission", "1".to_string());
        data.insert("EncryptedPasswd", master_token);
        data.insert("service", "oauth2:https://www.googleapis.com/auth/drive.appdata".to_string());
        data.insert("source", "android".to_string());
        data.insert("androidId", android_id.trim().to_string());
        data.insert("app", "com.whatsapp".to_string());
        data.insert("client_sig", "38a0f7d505fe18fec64fbf343ecaaaf310dbd799".to_string());
        data.insert("device_country", "us".to_string());
        data.insert("operatorCountry", "us".to_string());
        data.insert("lang", "en".to_string());
        data.insert("sdk_version", "17".to_string());
        data.insert("google_play_services_version", "240913000".to_string());

        let res_map = perform_auth_request(&client, &data).await?;
        if let Some(auth) = res_map.get("Auth") {
            Ok(Self {
                gmail: gmail.to_string(),
                android_id: android_id.to_string(),
                auth_token: auth.clone(),
                client,
            })
        } else {
            Err(format!("OAuth failed: {:?}", res_map).into())
        }
    }

    pub async fn get(&self, path: &str, params: Option<&[(&str, &str)]>) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("https://backup.googleapis.com/v1/{}", path);
        let auth_header = format!("Bearer {}", self.auth_token);
        
        let mut attempts = 0;
        let max_attempts = 5;
        let mut delay = std::time::Duration::from_secs(1);
        
        loop {
            let mut req = self.client.get(&url)
                .header("Authorization", &auth_header);
            if let Some(p) = params {
                req = req.query(p);
            }
            
            match req.send().await {
                Ok(resp) if resp.status().is_success() => return Ok(resp),
                Ok(resp) if resp.status().is_server_error() && attempts < max_attempts => {
                    attempts += 1;
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                }
                Err(e) if attempts < max_attempts => {
                    attempts += 1;
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                }
                res => return res,
            }
        }
    }

    pub async fn get_page(&self, path: &str, page_token: Option<&str>) -> Result<serde_json::Value, reqwest::Error> {
        let mut params = Vec::new();
        if let Some(token) = page_token {
            params.push(("pageToken", token));
        }
        let resp = self.get(path, if params.is_empty() { None } else { Some(&params) }).await?;
        resp.json().await
    }

    pub async fn list_path(&self, path: &str) -> Result<Vec<serde_json::Value>, reqwest::Error> {
        let last_component = path.split('/').last().unwrap_or("");
        let mut items = Vec::new();
        let mut page_token: Option<String> = None;
        
        loop {
            let page = self.get_page(path, page_token.as_deref()).await?;
            if let Some(arr) = page.get(last_component).and_then(|v| v.as_array()) {
                items.extend(arr.clone());
            }
            if let Some(next) = page.get("nextPageToken").and_then(|v| v.as_str()) {
                page_token = Some(next.to_string());
            } else {
                break;
            }
        }
        
        Ok(items)
    }

    pub async fn backups(&self) -> Result<Vec<BackupItem>, reqwest::Error> {
        let raw_items = self.list_path("clients/wa/backups").await?;
        let items = raw_items.into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect();
        Ok(items)
    }

    pub async fn backup_files(&self, backup_name: &str) -> Result<Vec<FileItem>, reqwest::Error> {
        let path = format!("{}/files", backup_name);
        let raw_items = self.list_path(&path).await?;
        let items = raw_items.into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect();
        Ok(items)
    }

    pub async fn fetch_file_bytes(&self, google_file_name: &str) -> Result<Vec<u8>, reqwest::Error> {
        let safe_name = google_file_name.replace("%", "%25").replace("+", "%2B");
        let resp = self.get(&safe_name, Some(&[("alt", "media")])).await?;
        resp.bytes().await.map(|b| b.to_vec())
    }
}

async fn perform_auth_request(
    client: &reqwest::Client,
    data: &HashMap<&str, String>
) -> Result<HashMap<String, String>, reqwest::Error> {
    let res = client.post(AUTH_URL)
        .header("Accept-Encoding", "identity")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("User-Agent", USER_AGENT)
        .form(data)
        .send()
        .await?
        .text()
        .await?;
        
    let mut map = HashMap::new();
    for line in res.lines() {
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.to_string(), v.to_string());
        }
    }
    Ok(map)
}

fn encrypt_password(email: &str, password: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let der_bytes = base64::engine::general_purpose::STANDARD.decode(B64_KEY_7_3_29)?;
    
    let mod_len = u32::from_be_bytes(der_bytes[0..4].try_into()?) as usize;
    let modulus_bytes = &der_bytes[4..4 + mod_len];
    
    let exp_offset = 4 + mod_len;
    let exp_len = u32::from_be_bytes(der_bytes[exp_offset..exp_offset + 4].try_into()?) as usize;
    let exponent_bytes = &der_bytes[exp_offset + 4 .. exp_offset + 4 + exp_len];
    
    let modulus = BigUint::from_bytes_be(modulus_bytes);
    let exponent = BigUint::from_bytes_be(exponent_bytes);
    
    let public_key = RsaPublicKey::new(modulus, exponent)?;
    
    let mut input = Vec::new();
    input.extend_from_slice(email.as_bytes());
    input.push(0x00);
    input.extend_from_slice(password.as_bytes());
    
    let mut rng = rand::thread_rng();
    let oaep = Oaep::new::<Sha1>();
    let encrypted = public_key.encrypt(&mut rng, oaep, &input)?;
    
    let mut signature = Vec::new();
    signature.push(0x00);
    signature.extend_from_slice(KEY_HASH_PREFIX);
    signature.extend_from_slice(&encrypted);
    
    Ok(base64::engine::general_purpose::URL_SAFE.encode(&signature))
}

pub fn human_size(size: i64) -> String {
    let units = ["B", "kiB", "MiB", "GiB", "TiB"];
    let mut size_f = size as f64;
    let mut unit_idx = 0;
    
    while size_f >= 1024.0 && unit_idx < units.len() - 1 {
        size_f /= 1024.0;
        unit_idx += 1;
    }
    
    format!("{:.0}{}", size_f, units[unit_idx])
}

pub fn get_backup_info(backup: &BackupItem) -> BackupInfo {
    let metadata: serde_json::Value = serde_json::from_str(&backup.metadata).unwrap_or(serde_json::Value::Null);
    let backup_size_str = metadata.get("backupSize").and_then(|v| v.as_str()).unwrap_or("0");
    let backup_size = backup_size_str.parse::<i64>().unwrap_or(0);
    
    BackupInfo {
        id: backup.name.split('/').last().unwrap_or("").to_string(),
        backupSize: backup_size_str.to_string(),
        backupSizeHuman: human_size(backup_size),
        uploadTime: backup.updateTime.clone(),
        whatsappVersion: metadata.get("versionOfAppWhenBackup").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
        messages: metadata.get("numOfMessages").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
        mediaFiles: metadata.get("numOfMediaFiles").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
        photos: metadata.get("numOfPhotos").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
    }
}

pub fn decrypt_db_bytes(key_hex: &str, crypt_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let sanitized_key = key_hex.replace(" ", "").trim().to_string();
    if sanitized_key.len() != 64 {
        return Err("La clave E2EE debe tener 64 caracteres hexadecimales".into());
    }
    
    let key_bytes = hex::decode(sanitized_key)?;
    let header_size = 191;
    if crypt_bytes.len() < header_size {
        return Err("Archivo demasiado pequeño para ser un backup válido".into());
    }
    
    let iv = &crypt_bytes[header_size - 16 .. header_size];
    let ciphertext = &crypt_bytes[header_size ..];
    
    type Aes256Gcm16 = AesGcm<Aes256, U16>;
    
    let key = aes_gcm::Key::<Aes256Gcm16>::from_slice(&key_bytes);
    let cipher = Aes256Gcm16::new(key);
    let nonce = aes_gcm::Nonce::<U16>::from_slice(iv);
    
    // WhatsApp's database decrypts with decrypt (ignoring missing tag errors if not strict)
    let decrypted = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Error en desencriptación: {:?}", e))?;
        
    Ok(decrypted)
}
