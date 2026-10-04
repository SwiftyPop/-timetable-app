pub mod commands;

use commands::wallpaper::set_desktop_wallpaper;
#[cfg(desktop)]
use commands::tray::setup_tray;
use commands::tray::update_tray_status;
use commands::notifications::notify_class_start;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(desktop)]
            setup_tray(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_desktop_wallpaper,
            update_tray_status,
            notify_class_start
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
