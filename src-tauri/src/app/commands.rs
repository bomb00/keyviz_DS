use std::sync::Mutex;

use tauri::{Manager, PhysicalPosition, PhysicalSize};
use tauri_plugin_store::StoreExt;

use crate::app::state::AppState;

#[tauri::command]
pub fn log(message: String) {
    println!("[LOG] {}", message);
}

#[tauri::command]
pub fn set_toggle_shortcut(app: tauri::AppHandle, shortcut: Vec<String>) {
    let state = app.state::<Mutex<AppState>>();
    let mut app_state = state.lock().unwrap();
    app_state.toggle_shortcut = shortcut;
}

#[tauri::command]
pub fn set_main_window_monitor(app: tauri::AppHandle, monitor_name: String) {
    let state = app.state::<Mutex<AppState>>();
    let mut app_state = state.lock().unwrap();

    if app_state.monitor_name == Some(monitor_name.clone()) {
        return;
    }

    if let Some(window) = app.get_webview_window("main") {
        let monitors = window.available_monitors().unwrap_or_default();
        let target_monitor = monitors.iter().find(|m| m.name() == Some(&monitor_name));

        if let Some(monitor) = target_monitor {
            let position = monitor.position();
            let size = monitor.size();
            let scale = monitor.scale_factor();

            // Update AppState
            app_state.monitor_name = Some(monitor_name.clone());
            app_state.monitor_scale = scale;
            app_state.monitor_position = (position.x, position.y);
            app_state.monitor_size = (size.width, size.height);

            // Update window
            window
                .set_position(PhysicalPosition {
                    x: position.x,
                    y: position.y,
                })
                .unwrap_or(());
            window
                .set_size(PhysicalSize {
                    width: size.width,
                    height: size.height,
                })
                .unwrap_or(());
        }
    }
}

// 활성 모니터 따라가기(커서가 있는 모니터에 표시) on/off
#[tauri::command]
pub fn set_follow_cursor(app: tauri::AppHandle, enabled: bool) {
    let state = app.state::<Mutex<AppState>>();
    let mut app_state = state.lock().unwrap();
    app_state.follow_cursor = enabled;
}

#[tauri::command]
pub fn get_dock_icon_visibility(app: tauri::AppHandle) -> bool {
    app.store("store.json")
        .ok()
        .and_then(|store| store.get("show_dock_icon"))
        .and_then(|value| value.as_bool())
        .unwrap_or(true)
}

#[tauri::command]
pub fn set_dock_icon_visibility(app: tauri::AppHandle, visible: bool) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    app.set_dock_visibility(visible)
        .map_err(|error| error.to_string())?;

    let store = app.store("store.json").map_err(|error| error.to_string())?;
    store.set("show_dock_icon", visible);
    store.save().map_err(|error| error.to_string())
}
