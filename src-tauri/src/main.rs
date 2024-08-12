// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod audio;
mod utils;

use audio::AudioPlayer;
use std::sync::Arc;
use std::sync::Mutex;

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

#[tauri::command]
fn play_playlist(file_path: Vec<String>, state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.play_playlist(file_path);
}

#[tauri::command]
fn next_track(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    println!("Next track");
    audio_player.next_track();
}

#[tauri::command]
fn previous_track(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    println!("Previous track");
    audio_player.previous_track();
}

#[tauri::command]
fn pause_audio(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.pause_audio();
}

#[tauri::command]
fn resume_audio(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.resume_audio();
}

#[tauri::command]
fn get_volume(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Option<f32> {
    let audio_player = state.lock().unwrap();
    audio_player.get_volume()
}

#[tauri::command]
fn set_volume(volume: f32, state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.set_volume(volume);
}

#[tauri::command]
fn get_audio_length(music_index: usize, state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> u64 {
    let audio_player = state.lock().unwrap();
    audio_player.get_audio_length(music_index)
}

#[tauri::command]
fn get_current_position(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Option<u64> {
    let audio_player = state.lock().unwrap();
    // println!("{:?}", audio_player.get_current_position());
    audio_player.get_current_position()
}

#[tauri::command]
fn get_current_music_index(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> usize {
    let audio_player = state.lock().unwrap();
    audio_player.get_current_music_index()
}

#[tauri::command]
fn set_current_time(position: u64, state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.set_current_time(position);
}

fn main() {
    let audio_player = Arc::new(Mutex::new(AudioPlayer::new()));
    audio_player.lock().unwrap().initialize();

    tauri::Builder::default()
        .manage(audio_player)
        .on_window_event(|event| {
            if let tauri::WindowEvent::Destroyed = event.event() {
                let window = event.window();
                if window.label() == "main" {
                    // audio_player.lock().unwrap().stop_audio();
                    window.app_handle().exit(256);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            create_settings,
            resize_window,
            play_playlist,
            pause_audio,
            resume_audio,
            next_track,
            previous_track,
            get_volume,
            set_volume,
            get_audio_length,
            get_current_position,
            get_current_music_index,
            set_current_time,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
