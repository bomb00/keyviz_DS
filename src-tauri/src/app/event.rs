use std::{sync::Mutex, thread};

use rdev::{listen, Button, EventType};
use serde::Serialize;
use tauri::{menu::MenuItem, AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Wry};

use crate::app::state::AppState;

// macOS: 비밀번호 등 보안 입력 필드 포커스 시 키 시각화를 억제하기 위한 판정
#[cfg(target_os = "macos")]
#[link(name = "Carbon", kind = "framework")]
extern "C" {
    fn IsSecureEventInputEnabled() -> u8;
}

#[cfg(target_os = "macos")]
fn is_secure_input() -> bool {
    unsafe { IsSecureEventInputEnabled() != 0 }
}

#[cfg(not(target_os = "macos"))]
fn is_secure_input() -> bool {
    false
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum InputEvent {
    KeyEvent { pressed: bool, name: String },
    MouseButtonEvent { pressed: bool, button: MouseButton },
    MouseMoveEvent { x: f64, y: f64 },
    MouseWheelEvent { delta_x: i64, delta_y: i64 },
}

#[derive(Debug, Clone, Serialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other,
}

pub fn map_mouse_button(button: Button) -> MouseButton {
    match button {
        Button::Left => MouseButton::Left,
        Button::Right => MouseButton::Right,
        Button::Middle => MouseButton::Middle,
        _ => MouseButton::Other,
    }
}

// 커서가 현재 모니터를 벗어났으면 창을 커서가 있는 모니터로 옮기고 상태를 갱신한다.
// rdev 좌표는 좌표계(논리/물리)가 불명확하므로 쓰지 않고, Tauri 물리 좌표
// (cursor_position)로 판정한다. 모니터 목록은 매번 라이브로 조회해 구성 변경에 대응한다.
fn reposition_if_needed(app_handle: &AppHandle, app_state: &mut AppState) {
    // Tauri 물리 좌표계의 커서 위치 (모니터 좌표와 동일 공간)
    let cursor = match app_handle.cursor_position() {
        Ok(p) => p,
        Err(_) => return,
    };
    let (mx, my) = (cursor.x as i32, cursor.y as i32);

    let (px, py) = app_state.monitor_position;
    let (sw, sh) = app_state.monitor_size;
    let inside = sw > 0
        && sh > 0
        && mx >= px
        && mx < px + sw as i32
        && my >= py
        && my < py + sh as i32;
    if inside {
        return;
    }

    if let Some(window) = app_handle.get_webview_window("main") {
        if let Ok(monitors) = window.available_monitors() {
            let target = monitors.iter().find(|m| {
                let pos = m.position();
                let size = m.size();
                mx >= pos.x
                    && mx < pos.x + size.width as i32
                    && my >= pos.y
                    && my < pos.y + size.height as i32
            });
            if let Some(monitor) = target {
                let pos = monitor.position();
                let size = monitor.size();
                app_state.monitor_name = monitor.name().cloned();
                app_state.monitor_scale = monitor.scale_factor();
                app_state.monitor_position = (pos.x, pos.y);
                app_state.monitor_size = (size.width, size.height);
                let _ = window.set_position(PhysicalPosition { x: pos.x, y: pos.y });
                let _ = window.set_size(PhysicalSize {
                    width: size.width,
                    height: size.height,
                });
            }
        }
    }
}

pub fn start_listener(app_handle: AppHandle, toggle_menu_item: MenuItem<Wry>) {
    thread::spawn(move || {
        println!("Starting global input listener...");

        if let Err(err) = listen(move |event| {
            // get app state
            let state = app_handle.state::<Mutex<AppState>>();
            let mut app_state = state.lock().unwrap();

            // track pressed keys
            if let EventType::KeyPress(key) = event.event_type {
                let key_name = format!("{:?}", key);
                // If the name contains parenthesis (like "RawKey(123)", "Unknown()"), ignore it.
                if key_name.contains('(') {
                    return;
                }
                // if key is already marked as pressed, ignore repeat
                if app_state.pressed_keys.contains(&key_name) {
                    return;
                }
                // record key as pressed
                app_state.pressed_keys.push(key_name);
                // check if toggle shortcut is pressed
                if app_state.toggle_shortcut == app_state.pressed_keys {
                    app_state.toggle_listener(&app_handle, &toggle_menu_item);

                    if !app_state.listening {
                        // emit key releases for all pressed keys
                        for key_name in &app_state.pressed_keys {
                            app_handle
                                .emit_to(
                                    "main",
                                    "input-event",
                                    InputEvent::KeyEvent {
                                        pressed: false,
                                        name: key_name.clone(),
                                    },
                                )
                                .unwrap()
                        }
                    }
                }
            } else if let EventType::KeyRelease(key) = event.event_type {
                let key_name = format!("{:?}", key);
                if key_name.contains('(') {
                    return;
                }
                // remove key from pressed keys
                app_state.pressed_keys.retain(|k| k != &key_name);
            }

            // emit event if listening
            if !app_state.listening {
                return;
            }

            // 활성 모니터 따라가기: 커서가 현재 모니터 밖으로 나가면 창을 그 모니터로 이동
            if app_state.follow_cursor {
                if let EventType::MouseMove { .. } = event.event_type {
                    reposition_if_needed(&app_handle, &mut app_state);
                }
            }

            let input_event = match event.event_type {
                EventType::KeyPress(key) => Some(InputEvent::KeyEvent {
                    pressed: true,
                    name: format!("{:?}", key),
                }),
                EventType::KeyRelease(key) => Some(InputEvent::KeyEvent {
                    pressed: false,
                    name: format!("{:?}", key),
                }),
                EventType::ButtonPress(button) => Some(InputEvent::MouseButtonEvent {
                    pressed: true,
                    button: map_mouse_button(button),
                }),
                EventType::ButtonRelease(button) => Some(InputEvent::MouseButtonEvent {
                    button: map_mouse_button(button),
                    pressed: false,
                }),
                EventType::MouseMove { x, y } => {
                    // Convert Physical -> Logical
                    #[cfg(target_os = "macos")]
                    let (logical_x, logical_y) = (
                        x - app_state.monitor_position.0 as f64,
                        y - app_state.monitor_position.1 as f64,
                    );

                    #[cfg(not(target_os = "macos"))]
                    let (logical_x, logical_y) = {
                        let (offset_x, offset_y) = app_state.monitor_position;
                        (x - offset_x as f64, y - offset_y as f64)
                    };

                    Some(InputEvent::MouseMoveEvent {
                        x: logical_x,
                        y: logical_y,
                    })
                }
                EventType::Wheel { delta_x, delta_y } => {
                    Some(InputEvent::MouseWheelEvent { delta_x, delta_y })
                }
            };

            // 보안 입력(비밀번호창 등) 중에는 키를 시각화하지 않는다
            if !is_secure_input() {
                app_handle.emit("input-event", input_event).unwrap();
            }
        }) {
            eprintln!("rdev listen failed: {:?}", err);
        }
    });
}
