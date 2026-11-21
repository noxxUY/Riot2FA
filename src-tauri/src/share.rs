use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE as B64URL;
use flate2::read::ZlibDecoder;
use serde::Deserialize;

use crate::vault::Account;

const PREFIX: &str = "RIOT2FA1-";

#[derive(Deserialize)]
struct SharePayload {
    name: Option<String>,
    seed: String,

}

#[tauri::command]
pub fn share_import(code: String) -> Result<Account, String> {
    let code = code.trim();
    let packed = code
        .strip_prefix(PREFIX)
        .ok_or("Eso no parece un código de Riot2FA.".to_string())?;

    let compressed = B64URL
        .decode(packed.trim())
        .map_err(|_| "El código está dañado o incompleto.".to_string())?;

    let mut raw = String::new();
    ZlibDecoder::new(&compressed[..])
        .read_to_string(&mut raw)
        .map_err(|_| "El código está dañado o incompleto.".to_string())?;

    let payload: SharePayload =
        serde_json::from_str(&raw).map_err(|_| "El código no tiene el formato esperado.".to_string())?;

    let seed = payload.seed.trim().to_string();
    if seed.is_empty() {
        return Err("El código no trae la clave de la cuenta.".to_string());
    }

    let name = payload
        .name
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "Cuenta importada".to_string());

    let region = name
        .split_once('#')
        .map(|(_, tag)| tag.to_uppercase())
        .unwrap_or_else(|| "LAS".to_string());

    Ok(Account {
        id: format!("a{}", now_ms()),
        riot_id: name,
        label: String::new(),
        region,
        seed,
    })
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}
