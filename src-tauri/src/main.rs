// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod utils;

use tauri::{Manager, Window};
use utils::{CLOSED_DIMENSIONS, OPENED_DIMENSIONS};

#[tauri::command]
async fn create_settings(handle: tauri::AppHandle) -> Result<(), tauri::Error> {
    let _ = tauri::WindowBuilder::new(
        &handle,
        "settings",
        tauri::WindowUrl::App("settings.html".into()),
    )
    .decorations(false)
    .resizable(false)
    .transparent(true)
    .build()?;
    Ok(())
}

#[tauri::command]
fn resize_window(window: Window) -> Result<(), tauri::Error> {
    let window_size = window.inner_size()?;
    match window_size {
        CLOSED_DIMENSIONS => window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: OPENED_DIMENSIONS.width as f64,
            height: OPENED_DIMENSIONS.height as f64,
        })),
        OPENED_DIMENSIONS => window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: CLOSED_DIMENSIONS.width as f64,
            height: CLOSED_DIMENSIONS.height as f64,
        })),
        _ => Ok(()),
    }
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Destroyed => {
                let window = event.window();
                if window.label() == "main" {
                    let _ = window.app_handle().exit(256);
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![create_settings, resize_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
