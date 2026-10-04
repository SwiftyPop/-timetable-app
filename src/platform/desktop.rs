// Windows Native Platform Integration (Wallpaper & Tray)

#[cfg(not(target_arch = "wasm32"))]
pub fn set_desktop_wallpaper(bytes: &[u8]) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;
    
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
    };

    let temp_dir = std::env::temp_dir();
    let wallpaper_path = temp_dir.join("unimap_timetable_wallpaper.png");

    let mut file = File::create(&wallpaper_path).map_err(|e| e.to_string())?;
    file.write_all(bytes).map_err(|e| e.to_string())?;
    drop(file);

    let path_str = wallpaper_path.to_str().ok_or("Invalid path string")?;
    let wide_path: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(wide_path.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if result.is_ok() {
            Ok(())
        } else {
            Err("SystemParametersInfoW failed to set wallpaper".to_string())
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_desktop_wallpaper(_bytes: &[u8]) -> Result<(), String> {
    Err("Not supported on WebAssembly".to_string())
}
