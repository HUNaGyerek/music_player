use std::sync::{Arc, Mutex};
use tauri::{Emitter, Runtime};

use crate::{
    audio::{AudioPlayer, Music},
    utils::{CLOSED_DIMENSIONS, OPENED_DIMENSIONS},
};

#[tauri::command]
pub async fn toggle_playlist_menu<R: Runtime>(
    // app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), tauri::Error> {
    let window_size = window.inner_size()?;
    println!("{window_size:?}");
    match window_size {
        CLOSED_DIMENSIONS => window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: 816_f64,
            height: 510_f64,
        })),
        OPENED_DIMENSIONS => window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: 816_f64,
            height: 204_f64,
        })),
        _ => Ok(()),
    }
}

#[tauri::command]
pub async fn play_by_index(
    state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>,
    idx: usize,
) -> Result<(), String> {
    let mut audio_player = state.lock().unwrap();
    audio_player.play_by_index(idx);
    Ok(())
}

#[tauri::command]
pub fn next_track(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.next();
}

#[tauri::command]
pub fn previous_track(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.previous();
}

#[tauri::command]
pub fn pause_audio(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.pause();
}

#[tauri::command]
pub fn resume_audio(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.resume();
}

#[tauri::command]
pub fn get_volume(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Option<f32> {
    let audio_player = state.lock().unwrap();
    println!("{:?}", audio_player.get_volume());
    audio_player.get_volume()
}

#[tauri::command]
pub fn set_volume(volume: f32, state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.set_volume(volume);
}

#[tauri::command]
pub fn get_current_track_informations(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Music {
    let audio_player = state.lock().unwrap();
    Music {
        title: audio_player.get_music_name(),
        lenght: audio_player.get_length().unwrap(),
        thumbnail: audio_player.get_thumbnail(),
    }
}

#[tauri::command]
pub fn get_current_audio_length(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> u64 {
    let audio_player = state.lock().unwrap();
    audio_player.get_length().unwrap()
}

#[tauri::command]
pub fn get_current_position(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Option<u64> {
    let audio_player = state.lock().unwrap();
    audio_player.get_position()
}

#[tauri::command]
pub fn get_current_music_index(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> usize {
    let audio_player = state.lock().unwrap();
    audio_player.get_current_index()
}

#[tauri::command]
pub async fn get_playlist(
    state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Result<Vec<Music>, String> {
    let audio_player = state.lock().unwrap();
    Ok(audio_player.get_playlist())
}

#[tauri::command]
pub fn set_current_time(position: u64, state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.set_current_time(position);
}

#[tauri::command]
pub fn get_playlist_len(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> usize {
    let audio_player = state.lock().unwrap();
    audio_player.get_playlist_len()
}

// Not Used
#[tauri::command]
pub fn get_current_track_name(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Option<String> {
    let audio_player = state.lock().unwrap();
    if audio_player.get_playlist_len() != 0 {
        Some(audio_player.get_music_name())
    } else {
        None
    }
}

#[tauri::command]
pub fn shuffle_playlist(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.toggle_shuffle();
}

pub fn start_autonomous_playback(app_handle: tauri::AppHandle, state: Arc<Mutex<AudioPlayer>>) {
    std::thread::spawn(move || {
        let mut last_position: Option<u64> = None; // Tracks the last reported position
        let mut last_index: Option<usize> = None; // Tracks the last played track index
        loop {
            let mut player = state.lock().unwrap();

            // Get the current track index
            let current_index = player.get_current_index();

            // Handle manual track skips or changes
            if last_index.is_none() || last_index != Some(current_index) {
                // Track index changed (manual skip/previous)
                last_index = Some(current_index);
                last_position = None; // Reset position tracking
                app_handle.emit("track-changed", ()).unwrap();
            }

            // Handle progress updates
            if let Some(position) = player.get_position() {
                if last_position.is_none() || last_position != Some(position) {
                    // Position changed (either natural or manual seek)
                    last_position = Some(position);
                    app_handle.emit("track-progress", position).unwrap();
                }

                // Check if the track has finished
                if let Some(duration) = player.get_length() {
                    if position >= duration {
                        player.next(); // Move to the next track
                        last_index = None; // Reset index tracking
                        last_position = None; // Reset position tracking
                        app_handle.emit("track-changed", ()).unwrap();
                    }
                }
            }

            drop(player); // Release the lock before sleeping
            std::thread::sleep(std::time::Duration::from_millis(500)); // Check progress every halfsec
        }
    });
}
