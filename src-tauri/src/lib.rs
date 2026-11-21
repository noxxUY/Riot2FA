mod biometric;
mod riot_api;
mod riot_login;
mod share;
mod totp;
mod vault;

use serde::Serialize;

#[derive(Serialize)]
pub struct CodeResult {

    code: String,

    seconds_remaining: u64,
}

#[tauri::command]
fn totp_code(seed: String) -> Result<CodeResult, String> {
    match totp::current_code(&seed) {
        Some(code) => Ok(CodeResult {
            code,
            seconds_remaining: totp::seconds_remaining(),
        }),
        None => Err("Seed inválida".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            totp_code,
            riot_login::start_riot_login,
            vault::vault_load,
            vault::vault_save,
            share::share_import,
            biometric::biometric_unlock
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
