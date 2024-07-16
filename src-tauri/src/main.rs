// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod utils;

use tauri::Window;
use utils::{CLOSED_DIMENSIONS, OPENED_DIMENSIONS};

#[tauri::command]
fn toggle_settings_menu(window: Window) -> Result<(), tauri::Error> {
    let window_size = window.inner_size().unwrap();
    match window_size {
        CLOSED_DIMENSIONS => window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: OPENED_DIMENSIONS.width as f64,
            height: OPENED_DIMENSIONS.height as f64,
        })),
        _ => Ok(()),
    }?;
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
        .invoke_handler(tauri::generate_handler![
            toggle_settings_menu,
            resize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
