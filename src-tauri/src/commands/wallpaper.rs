use tauri::command;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE,
};
#[cfg(target_os = "windows")]
use windows::core::HSTRING;

#[command]
pub async fn set_desktop_wallpaper(image_bytes: Vec<u8>) -> Result<String, String> {
    let temp_dir = std::env::temp_dir();
    let wallpaper_path = temp_dir.join("unimap_timetable_wallpaper.png");

    std::fs::write(&wallpaper_path, image_bytes)
        .map_err(|e| format!("Failed to save wallpaper cache: {}", e))?;

    #[cfg(target_os = "windows")]
    unsafe {
        let path_str = wallpaper_path.to_string_lossy().to_string();
        let h_path = HSTRING::from(path_str);
        match SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(h_path.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) {
            Ok(()) => Ok("Desktop wallpaper updated successfully".to_string()),
            Err(e) => Err(format!("Win32 SystemParametersInfoW call failed: {}", e)),
        }
    }

    #[cfg(not(target_os = "windows"))]
    Err("Direct wallpaper setting is currently supported on Windows".to_string())
}
