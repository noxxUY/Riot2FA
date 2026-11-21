use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::vault::Account;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36";

pub struct AccountSession {
    pub cookies: BTreeMap<String, String>,
    pub csrf: String,
}

#[derive(Clone, serde::Serialize)]
pub struct AccountInfo {
    pub riot_id: String,
    pub puuid: Option<String>,
}

#[derive(Deserialize)]
struct Alias {
    game_name: Option<String>,
    tag_line: Option<String>,
}

#[derive(Deserialize)]
struct AccountUser {
    alias: Option<Alias>,
    puuid: Option<String>,
    sub: Option<String>,
    username: Option<String>,
}

pub fn fetch_account_info(session: &AccountSession) -> Result<AccountInfo, String> {
    let cookie_header = session
        .cookies
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("; ");

    let client = reqwest::blocking::Client::builder()
        .build()
        .map_err(|e| format!("No se pudo crear el cliente HTTP: {e}"))?;

    let resp = client
        .get("https://account.riotgames.com/api/account/v1/user")
        .header("accept", "application/json")
        .header("csrf-token", &session.csrf)
        .header("cookie", cookie_header)
        .header("origin", "https://account.riotgames.com")
        .header("referer", "https://account.riotgames.com/")
        .header("user-agent", USER_AGENT)
        .send()
        .map_err(|e| format!("Falló la petición: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("Riot respondió HTTP {status}"));
    }

    let user: AccountUser = resp
        .json()
        .map_err(|e| format!("Respuesta inesperada de Riot: {e}"))?;

    let riot_id = riot_id_from(&user);
    let puuid = user.puuid.clone().or_else(|| user.sub.clone());

    Ok(AccountInfo { riot_id, puuid })
}

const FACTORS_URL: &str = "https://account.riotgames.com/api/mfa/v2/factors";
const ENABLE_URL: &str =
    "https://account.riotgames.com/api/mfa/v2/factors/riotmobile/enable";

fn cookie_header(session: &AccountSession) -> String {
    session
        .cookies
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("; ")
}

fn fetch_factors(session: &AccountSession) -> Result<serde_json::Value, String> {
    let client = reqwest::blocking::Client::builder()
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .get(FACTORS_URL)
        .header("accept", "application/json")
        .header("csrf-token", &session.csrf)
        .header("cookie", cookie_header(session))
        .header("origin", "https://account.riotgames.com")
        .header("referer", "https://account.riotgames.com/")
        .header("user-agent", USER_AGENT)
        .send()
        .map_err(|e| format!("Falló la consulta de factores MFA: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Riot respondió HTTP {} al leer los factores", resp.status()));
    }
    resp.json().map_err(|e| e.to_string())
}

fn email_mfa_enabled(factors: &serde_json::Value) -> bool {
    let items = if factors.is_array() {
        factors.as_array()
    } else {
        factors.get("factors").and_then(|f| f.as_array())
    };
    if let Some(arr) = items {
        for f in arr {
            if f.get("factor").and_then(|v| v.as_str()) == Some("email") {
                return f.get("status").and_then(|v| v.as_str()) == Some("enabled");
            }
        }
    }
    false
}

fn enable_riot_mobile(session: &AccountSession) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .post(ENABLE_URL)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .header("csrf-token", &session.csrf)
        .header("cookie", cookie_header(session))
        .header("origin", "https://account.riotgames.com")
        .header("referer", "https://account.riotgames.com/")
        .header("user-agent", USER_AGENT)
        .send()
        .map_err(|e| format!("Falló la activación de Riot Mobile: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!(
            "Riot respondió HTTP {} al activar Riot Mobile",
            resp.status()
        ));
    }
    let v: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    match v.get("secret").and_then(|s| s.as_str()) {
        Some(s) if !s.is_empty() => Ok(s.to_string()),
        _ => Err(
            "Riot no devolvió la clave: probablemente Riot Mobile ya está activo en esta \
             cuenta. Desactivalo en account.riotgames.com y reintentá, o importá la cuenta \
             con un código."
                .to_string(),
        ),
    }
}

pub fn provision_account(
    session: &AccountSession,
    riot_id: String,
    _puuid: Option<String>,
) -> Result<Account, String> {
    let factors = fetch_factors(session)?;
    if !email_mfa_enabled(&factors) {
        return Err(
            "Esta cuenta todavía no tiene la verificación en dos pasos por email. Activala en \
             account.riotgames.com (Seguridad → Multi-factor authentication) y volvé a intentar."
                .to_string(),
        );
    }
    let seed = enable_riot_mobile(session)?;
    let region = riot_id
        .split_once('#')
        .map(|(_, tag)| tag.to_uppercase())
        .unwrap_or_else(|| "—".to_string());

    Ok(Account {
        id: format!("a{}", now_ms()),
        riot_id,
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

fn riot_id_from(user: &AccountUser) -> String {
    if let Some(alias) = &user.alias {
        match (&alias.game_name, &alias.tag_line) {
            (Some(g), Some(t)) if !g.is_empty() && !t.is_empty() => return format!("{g}#{t}"),
            (Some(g), _) if !g.is_empty() => return g.clone(),
            _ => {}
        }
    }
    user.username
        .clone()
        .or_else(|| user.sub.clone())
        .unwrap_or_else(|| "Desconocido".to_string())
}
