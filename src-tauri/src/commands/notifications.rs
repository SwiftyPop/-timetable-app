use serde::Serialize;
use tauri::{command, AppHandle};
use tauri_plugin_notification::NotificationExt;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum NotificationError {
    #[error("Failed to show notification: {0}")]
    ShowFailed(String),
}

#[command]
pub fn notify_class_start(
    app: AppHandle,
    title: String,
    body: String,
) -> Result<(), NotificationError> {
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|e| NotificationError::ShowFailed(e.to_string()))?;
    Ok(())
}
