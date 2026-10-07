use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use thiserror::Error;

#[cfg(desktop)]
use tauri::menu::{Menu, MenuItem};
#[cfg(desktop)]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum TrayError {
    #[error("Tray icon not found")]
    NotFound,
    #[error("Failed to update tray status: {0}")]
    UpdateFailed(String),
}

pub struct TrayState {
    #[cfg(desktop)]
    pub status_item: MenuItem<tauri::Wry>,
    pub last_status: Mutex<String>,
}

#[cfg(desktop)]
pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Disabled item to mirror current status into the menu
    let status_item = MenuItem::with_id(app, "status", "Status: Ready", false, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "Open Timetable", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "Hide to Tray", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&status_item, &show_item, &hide_item, &quit_item])?;

    let _tray = TrayIconBuilder::with_id("main-tray")
        .tooltip("Campus Timetable · Ready")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
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

    app.manage(TrayState {
        status_item,
        last_status: Mutex::new(String::new()),
    });

    Ok(())
}

#[tauri::command]
pub fn update_tray_status(
    app: AppHandle,
    state: State<'_, TrayState>,
    status_text: String,
) -> Result<(), TrayError> {
    #[cfg(desktop)]
    {
        let trimmed = status_text.trim();
        let short_status = if trimmed.chars().count() > 80 {
            let s: String = trimmed.chars().take(80).collect();
            format!("{}…", s)
        } else {
            trimmed.to_string()
        };

        // Throttle IPC: check if status text changed
        let mut last = state
            .last_status
            .lock()
            .map_err(|e| TrayError::UpdateFailed(e.to_string()))?;

        if *last == short_status {
            return Ok(());
        }
        *last = short_status.clone();

        // Mirror into disabled tray menu item
        let _ = state.status_item.set_text(format!("Status: {}", short_status));

        // Update tooltip: Windows caps around 127 chars, keep concise
        if let Some(tray) = app.tray_by_id("main-tray") {
            let tooltip = format!("Campus Timetable · {}", short_status);
            let _ = tray.set_tooltip(Some(tooltip));
        }
    }

    #[cfg(not(desktop))]
    {
        let _ = (app, state, status_text);
    }

    Ok(())
}
