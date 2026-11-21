use std::collections::BTreeMap;
use std::sync::mpsc;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, Url, WebviewUrl, WebviewWindowBuilder};

use crate::riot_api::{self, AccountSession};

const LOGIN_LABEL: &str = "riot-login";
const POLL_INTERVAL_MS: u64 = 1500;
const MAX_POLLS: u32 = 400;

#[tauri::command]
pub async fn start_riot_login(app: AppHandle) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window(LOGIN_LABEL) {
        let _ = existing.set_focus();
        return Ok(());
    }

    let url: Url = "https://account.riotgames.com/"
        .parse()
        .map_err(|_| "URL de login inválida".to_string())?;

    WebviewWindowBuilder::new(&app, LOGIN_LABEL, WebviewUrl::External(url))
        .title("Iniciar sesión en Riot")
        .inner_size(480.0, 720.0)
        .center()

        .incognito(true)
        .build()
        .map_err(|e| e.to_string())?;

    let app_poll = app.clone();
    std::thread::spawn(move || poll_for_session(app_poll));
    Ok(())
}

struct Probe {
    has_ssid: bool,
    has_csrf: bool,

    account_cookies: BTreeMap<String, String>,

    csrf: Option<String>,
}

fn poll_for_session(app: AppHandle) {
    for _ in 0..MAX_POLLS {
        std::thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));

        let probe = match read_cookies_on_main(&app) {
            None => {
                let _ = app.emit("riot-login-cancelled", ());
                return;
            }
            Some(p) => p,
        };

        if probe.has_ssid && probe.has_csrf {
            if let Some(w) = app.get_webview_window(LOGIN_LABEL) {
                let _ = w.close();
            }
            let _ = app.emit("riot-login-captured", ());

            let session = AccountSession {
                cookies: probe.account_cookies,
                csrf: probe.csrf.unwrap_or_default(),
            };

            let (riot_id, puuid) = match riot_api::fetch_account_info(&session) {
                Ok(info) => (info.riot_id, info.puuid),
                Err(e) => {
                    let _ = app.emit("riot-add-error", e);
                    return;
                }
            };

            match riot_api::provision_account(&session, riot_id, puuid) {
                Ok(account) => {
                    let _ = app.emit("riot-account-added", account);
                }
                Err(e) => {
                    let _ = app.emit("riot-add-error", e);
                }
            }
            return;
        }
    }

    let _ = app.emit("riot-login-timeout", ());
    if let Some(w) = app.get_webview_window(LOGIN_LABEL) {
        let _ = w.close();
    }
}

fn read_cookies_on_main(app: &AppHandle) -> Option<Probe> {
    let (tx, rx) = mpsc::channel::<Option<Probe>>();
    let app_main = app.clone();

    let dispatched = app.run_on_main_thread(move || {
        let probe = (|| {
            let win = app_main.get_webview_window(LOGIN_LABEL)?;
            let auth: Url = "https://auth.riotgames.com".parse().ok()?;
            let account: Url = "https://account.riotgames.com".parse().ok()?;

            let auth_cookies = win.cookies_for_url(auth).ok()?;
            let account_cookies = win.cookies_for_url(account).ok()?;

            let has_ssid = auth_cookies.iter().any(|c| c.name() == "ssid");

            let mut account_map = BTreeMap::new();
            let mut csrf = None;
            for c in &account_cookies {
                account_map.insert(c.name().to_string(), c.value().to_string());
                if c.name() == "a12l-csrf-prod" {
                    csrf = Some(c.value().to_string());
                }
            }

            Some(Probe {
                has_ssid,
                has_csrf: csrf.is_some(),
                account_cookies: account_map,
                csrf,
            })
        })();
        let _ = tx.send(probe);
    });

    if dispatched.is_err() {
        return None;
    }
    rx.recv_timeout(Duration::from_secs(5)).ok().flatten()
}
