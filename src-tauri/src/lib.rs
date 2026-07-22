use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

use tauri::{
    image::Image,
    include_image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, WebviewWindowBuilder,
};

mod app;
use app::commands::{
    get_dock_icon_visibility, log, set_dock_icon_visibility, set_follow_cursor,
    set_main_window_monitor, set_toggle_shortcut,
};
use app::event::start_listener;
use app::state::AppState;
use app::window::config_window;

// 트레이가 없어도 설정 창을 열 수 있게 하는 공통 함수 (Dock 재열기·재실행 시 사용)
fn open_settings(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_focus();
        return;
    }
    let webview_url = tauri::WebviewUrl::App("index.html#/settings".into());
    if let Ok(window) = WebviewWindowBuilder::new(app, "settings", webview_url)
        .title("Keyviz")
        .inner_size(800.0, 640.0)
        .min_inner_size(640.0, 480.0)
        .max_inner_size(1000.0, 800.0)
        .maximizable(false)
        .build()
    {
        let _ = window.set_focus();
    }
    let _ = app.emit_to("main", "settings-window", true);
}

// 트레이·전역 리스너 초기화가 1회만 실행되도록 하는 플래그
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// 앱 기동 완료(RunEvent::Ready) 후 트레이 아이콘과 전역 입력 리스너를 초기화한다.
// macOS 에서 상태아이템은 앱 실행 완료 전(setup)에 만들면 렌더가 누락될 수 있어
// 이 시점에 생성한다.
fn setup_tray_and_listener(app: &AppHandle) {
    // Ready 는 1회만 발생하지만 안전하게 중복 초기화를 막는다
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return;
    }

    // tray actions
    let toggle_item = MenuItem::with_id(app, "toggle", "Stop", true, None::<&str>).unwrap();
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    // start global input listener
    start_listener(app.clone(), toggle_item.clone());

    // setup tray menu
    let menu = Menu::with_items(app, &[&toggle_item, &settings_item, &quit_item]).unwrap();
    let tray_result = TrayIconBuilder::with_id("keyviz-tray")
        .icon(Image::from(include_image!("icons/tray.png")))
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "toggle" => {
                let state = app.state::<Mutex<AppState>>();
                let mut app_state = state.lock().unwrap();
                app_state.toggle_listener(app, &toggle_item);
            }
            "settings" => open_settings(app),
            "quit" => std::process::exit(0),
            _ => {}
        })
        .build(app);
    if let Err(e) = &tray_result {
        eprintln!("tray icon build failed: {:?}", e);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 앱을 다시 실행(open -n 등)하면 설정 창이 열리는 우회 경로
            open_settings(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None::<Vec<&str>>,
        ))
        .plugin(tauri_plugin_prevent_default::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            if !get_dock_icon_visibility(app.handle().clone()) {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // prepare window
            if let Some(window) = app.get_webview_window("main") {
                config_window(&window);
            }

            let app_handle = app.handle();
            // manage app state
            app.manage(Mutex::new(AppState::new(&app_handle)));

            // 트레이·리스너는 앱 기동 완료(RunEvent::Ready) 후 초기화한다
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "settings" {
                return;
            }
            match event {
                tauri::WindowEvent::CloseRequested { .. } => {
                    window
                        .app_handle()
                        .emit_to("main", "settings-window", false)
                        .unwrap();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            log,
            set_toggle_shortcut,
            set_main_window_monitor,
            set_follow_cursor,
            get_dock_icon_visibility,
            set_dock_icon_visibility
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| match event {
            // 앱 기동 완료 후 트레이·리스너 초기화 (setup 단계보다 렌더가 안정적)
            tauri::RunEvent::Ready => setup_tray_and_listener(app),
            // Dock 아이콘 클릭 등 재열기 시 설정 창을 연다 (트레이가 없어도 접근 가능)
            tauri::RunEvent::Reopen { .. } => open_settings(app),
            _ => {}
        });
}
