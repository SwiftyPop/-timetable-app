pub mod commands;

use commands::notifications::notify_class_start;
#[cfg(desktop)]
use commands::tray::setup_tray;
use commands::tray::update_tray_status;
use commands::wallpaper::set_desktop_wallpaper;
use commands::open_external_url;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            #[cfg(desktop)]
            setup_tray(app.handle())?;

            // Close-to-tray: prevent window destroy, hide to tray instead
            if let Some(window) = app.get_webview_window("main") {
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = w.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_desktop_wallpaper,
            update_tray_status,
            notify_class_start,
            open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
