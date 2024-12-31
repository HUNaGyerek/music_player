use std::sync::{Arc, Mutex};

use audio::AudioPlayer;
use tauri::Manager;

mod audio;
mod commands;
mod utils;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_player = Arc::new(Mutex::new(AudioPlayer::new()));

    // Sense if its in release or dev mode
    let args: Vec<std::path::PathBuf> = if cfg!(debug_assertions) {
        // Dev mode
        println!("{:?}", "Dev mode");
        vec![
            std::path::PathBuf::from("Overtone.exe"),
            std::path::PathBuf::from(
                "C:\\Users\\Vivi-PC\\Music\\B3nte, Mike Emilio - 6 Little Eggs.mp3",
                // "D:\\Programozas\\Rust\\youtube-downloader\\WINX CLUB [bAMbi0E3AkI].mp3",
            ),
        ]
    } else {
        // Release mode
        println!("{:?}", "Release mode");
        std::env::args().map(std::path::PathBuf::from).collect()
    };

    if args.len() > 1 {
        let file_paths = args[1..].to_vec();
        let dir = std::path::Path::new(&file_paths[0]).parent().unwrap();
        let mut entries = std::fs::read_dir(dir)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();
        entries.sort();
        // println!("{:#?}", entries);

        let target = &args[1];
        let position = entries.iter().position(|entry| entry == target);

        audio_player
            .lock()
            .unwrap()
            .load_playlist(entries, position.unwrap());
    }

    tauri::Builder::default()
        .manage(audio_player)
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let state = app.state::<Arc<Mutex<AudioPlayer>>>().inner().clone();
            start_autonomous_playback(app.handle().clone(), state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            pause_audio,
            resume_audio,
            next_track,
            previous_track,
            get_volume,
            set_volume,
            get_current_position,
            get_current_music_index,
            set_current_time,
            get_playlist_len,
            get_current_track_name,
            shuffle_playlist,
            get_current_track_informations,
            get_current_audio_length,
            toggle_playlist_menu,
            get_playlist,
            play_by_index,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
