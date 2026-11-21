use std::sync::mpsc;

use robius_authentication::{
    AndroidText, BiometricStrength, Context, PolicyBuilder, Text, WindowsText,
};

#[tauri::command]
pub fn biometric_unlock(allow_biometrics: bool) -> Result<bool, String> {
    let text = Text {
        android: AndroidText {
            title: "Riot2FA",
            subtitle: None,
            description: None,
        },
        apple: "desbloquear Riot2FA",
        windows: WindowsText::new("Riot2FA", "Desbloquear Riot2FA")
            .ok_or_else(|| "Texto de autenticación inválido".to_string())?,
    };

    let policy = PolicyBuilder::new()
        .biometrics(if allow_biometrics {
            Some(BiometricStrength::Strong)
        } else {
            None
        })
        .password(true)
        .build()
        .ok_or_else(|| "No se pudo crear la política de autenticación".to_string())?;

    let (tx, rx) = mpsc::channel::<bool>();

    let ctx = Context::new(());
    ctx.authenticate(text, &policy, move |res| {
        let _ = tx.send(res.is_ok());
    })
    .map_err(|e| format!("No se pudo iniciar la autenticación: {e:?}"))?;

    rx.recv().map_err(|e| e.to_string())
}
