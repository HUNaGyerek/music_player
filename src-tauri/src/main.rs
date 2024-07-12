// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn resize_window(window: Window) {
    match !is_music_list_opened(window.clone()) {
        true => window
            .set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: 800f64,
                height: 600f64,
            }))
            .unwrap(),
        false => window
            .set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: 800f64,
                height: 200f64,
            }))
            .unwrap(),
    }
}

#[tauri::command]
fn is_music_list_opened(window: Window) -> bool {
    let ret = window.inner_size().unwrap();
    if ret.height > 200 {
        return true;
    }
    false
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            resize_window,
            is_music_list_opened
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
