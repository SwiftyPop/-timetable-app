pub mod notifications;
pub mod schedule;
pub mod tray;
pub mod wallpaper;

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        open::that(&url).map_err(|e| e.to_string())
    } else {
        Err("Blocked unsupported URL scheme".to_string())
    }
}
