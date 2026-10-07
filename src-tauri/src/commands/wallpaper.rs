use serde::Serialize;
use tauri::{command, AppHandle, Manager};
use thiserror::Error;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE,
};
#[cfg(target_os = "windows")]
use windows::core::HSTRING;

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const MAX_WALLPAPER_SIZE: usize = 20 * 1024 * 1024; // 20 MB limit

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum WallpaperError {
    #[error("Image format invalid: must be a valid PNG image")]
    InvalidFormat,
    #[error("Image payload too large (exceeds {0} bytes)")]
    PayloadTooLarge(usize),
    #[error("Invalid request payload: {0}")]
    InvalidPayload(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Platform error: {0}")]
    Platform(String),
}

#[cfg(target_os = "windows")]
fn set_wallpaper_fit_fill_style() -> Result<(), std::io::Error> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey_with_flags("Control Panel\\Desktop", KEY_SET_VALUE)?;
    key.set_value("WallpaperStyle", &"10")?; // 10 = Fill, 6 = Fit
    key.set_value("TileWallpaper", &"0")?;
    Ok(())
}

#[command]
pub async fn set_desktop_wallpaper(
    app: AppHandle,
    request: tauri::ipc::Request<'_>,
) -> Result<String, WallpaperError> {
    // 1. Extract raw bytes from IPC request
    let bytes: Vec<u8> = match request.body() {
        tauri::ipc::InvokeBody::Raw(raw) => raw.clone(),
        tauri::ipc::InvokeBody::Json(val) => {
            if let Some(arr) = val.as_array() {
                arr.iter().filter_map(|v| v.as_u64().map(|b| b as u8)).collect()
            } else if let Some(arr) = val.get("imageBytes").and_then(|v| v.as_array()) {
                arr.iter().filter_map(|v| v.as_u64().map(|b| b as u8)).collect()
            } else {
                return Err(WallpaperError::InvalidPayload("Expected binary or byte array".into()));
            }
        }
    };

    // 2. Size limit check
    if bytes.len() > MAX_WALLPAPER_SIZE {
        return Err(WallpaperError::PayloadTooLarge(MAX_WALLPAPER_SIZE));
    }

    // 3. PNG header verification
    if bytes.len() < PNG_HEADER.len() || &bytes[..PNG_HEADER.len()] != PNG_HEADER {
        return Err(WallpaperError::InvalidFormat);
    }

    // 4. Resolve app data directory (permanent cache, not OS-cleaned temp)
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| WallpaperError::Io(format!("Failed to resolve app data directory: {}", e)))?;

    // Ensure directory exists
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| WallpaperError::Io(format!("Failed to create app data directory: {}", e)))?;

    let wallpaper_path = app_dir.join("wallpaper.png");
    let save_path = wallpaper_path.clone();

    // 5. Write file off the UI thread
    tauri::async_runtime::spawn_blocking(move || std::fs::write(&save_path, bytes))
        .await
        .map_err(|e| WallpaperError::Io(format!("Task execution error: {}", e)))?
        .map_err(|e| WallpaperError::Io(format!("Failed to write wallpaper file: {}", e)))?;

    // 6. Set fit/fill style and apply wallpaper
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = set_wallpaper_fit_fill_style() {
            eprintln!("[Wallpaper] Warning: Failed to set wallpaper fit/fill registry keys: {}", e);
        }

        unsafe {
            let path_str = wallpaper_path.to_string_lossy().to_string();
            let h_path = HSTRING::from(path_str);
            SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                Some(h_path.as_ptr() as *mut _),
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            )
            .map_err(|e| WallpaperError::Platform(format!("Win32 SystemParametersInfoW failed: {}", e)))?;
        }

        Ok("Desktop wallpaper updated successfully".to_string())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(WallpaperError::Platform("Direct wallpaper setting is currently supported on Windows".to_string()))
    }
}
