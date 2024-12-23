use std::sync::{Arc, Mutex};

use crate::audio::{AudioPlayer, Music};

#[tauri::command]
pub fn next_track(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    // println!("Next track");
    audio_player.next_track();
}

#[tauri::command]
pub fn previous_track(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    // println!("Previous track");
    audio_player.previous_track();
}

#[tauri::command]
pub fn pause_audio(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.pause_audio();
}

#[tauri::command]
pub fn resume_audio(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.resume_audio();
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
pub fn get_audio_length(
    music_index: usize,
    state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Option<u64> {
    let audio_player = state.lock().unwrap();
    audio_player.get_audio_length(music_index)
}

#[tauri::command]
pub fn get_current_track_informations(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Music {
    let audio_player = state.lock().unwrap();
    Music {
        title: audio_player.get_current_track_name(),
        lenght: audio_player.get_current_audio_length().unwrap(),
    }
}

#[tauri::command]
pub fn get_current_position(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Option<u64> {
    let audio_player = state.lock().unwrap();
    // println!("{:?}", audio_player.get_current_position());
    audio_player.get_current_position()
}

#[tauri::command]
pub fn get_current_music_index(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> usize {
    let audio_player = state.lock().unwrap();
    audio_player.get_current_music_index()
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

#[tauri::command]
pub fn get_current_track_name(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) -> Option<String> {
    let audio_player = state.lock().unwrap();
    if audio_player.get_playlist_len() != 0 {
        Some(audio_player.get_current_track_name())
    } else {
        None
    }
}

#[tauri::command]
pub fn shuffle_playlist(state: tauri::State<'_, Arc<Mutex<AudioPlayer>>>) {
    let mut audio_player = state.lock().unwrap();
    audio_player.toggle_shuffle();
}
