use tauri::AppHandle;
#[cfg(desktop)]
use tauri::Manager;
#[cfg(desktop)]
use tauri::menu::{Menu, MenuItem};
#[cfg(desktop)]
use tauri::tray::TrayIconBuilder;

#[cfg(desktop)]
pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "Open Timetable", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "Hide to Tray", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

    let _tray = TrayIconBuilder::with_id("main-tray")
        .tooltip("UniMAP Timetable · Ready")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

#[tauri::command]
pub fn update_tray_status(app: AppHandle, status_text: String) -> Result<(), String> {
    #[cfg(desktop)]
    if let Some(tray) = app.tray_by_id("main-tray") {
        let _ = tray.set_tooltip(Some(format!("UniMAP Timetable · {}", status_text)));
    }
    #[cfg(not(desktop))]
    let _ = (app, status_text);
    Ok(())
}
