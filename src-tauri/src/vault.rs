use std::fs;
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use serde::{Deserialize, Serialize};
use tauri::Manager;

const KEYRING_SERVICE: &str = "com.riot2fa.app";
const KEYRING_KEY: &str = "vault-master-key";
const NONCE_LEN: usize = 24;
const KEY_LEN: usize = 32;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub riot_id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub region: String,
    pub seed: String,
}

fn vault_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("No se encontró el directorio de datos: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("vault.enc"))
}

fn master_key(create: bool) -> Result<Option<[u8; KEY_LEN]>, String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_KEY).map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(b64) => {
            let bytes = B64.decode(b64).map_err(|e| e.to_string())?;
            let arr: [u8; KEY_LEN] = bytes
                .try_into()
                .map_err(|_| "La clave del llavero tiene un tamaño inválido".to_string())?;
            Ok(Some(arr))
        }
        Err(keyring::Error::NoEntry) => {
            if !create {
                return Ok(None);
            }
            let mut key = [0u8; KEY_LEN];
            getrandom::getrandom(&mut key).map_err(|e| e.to_string())?;
            entry
                .set_password(&B64.encode(key))
                .map_err(|e| e.to_string())?;
            Ok(Some(key))
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn vault_load(app: tauri::AppHandle) -> Result<Vec<Account>, String> {
    let path = vault_path(&app)?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read(&path).map_err(|e| e.to_string())?;
    if data.len() <= NONCE_LEN {
        return Ok(vec![]);
    }
    let key = master_key(false)?.ok_or_else(|| {
        "Hay una bóveda pero no su clave en el llavero".to_string()
    })?;

    let (nonce, ct) = data.split_at(NONCE_LEN);
    let cipher =
        XChaCha20Poly1305::new_from_slice(&key).map_err(|e| e.to_string())?;
    let plaintext = cipher
        .decrypt(XNonce::from_slice(nonce), ct)
        .map_err(|_| "No se pudo descifrar la bóveda".to_string())?;

    serde_json::from_slice(&plaintext).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn vault_save(app: tauri::AppHandle, accounts: Vec<Account>) -> Result<(), String> {
    let path = vault_path(&app)?;
    let key = master_key(true)?.expect("create=true siempre devuelve clave");

    let mut nonce = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut nonce).map_err(|e| e.to_string())?;

    let plaintext = serde_json::to_vec(&accounts).map_err(|e| e.to_string())?;
    let cipher =
        XChaCha20Poly1305::new_from_slice(&key).map_err(|e| e.to_string())?;
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| "No se pudo cifrar la bóveda".to_string())?;

    let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    fs::write(&path, out).map_err(|e| e.to_string())?;
    Ok(())
}
