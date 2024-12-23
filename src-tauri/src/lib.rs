use std::sync::{Arc, Mutex};

use audio::AudioPlayer;

mod audio;
mod commands;
mod utils;

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
                "D:\\Programozas\\Rust\\youtube-downloader\\songs\\Wolverave - Vielleicht Vielleicht (Hardtekk Edit).mp3",
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
            .play_playlist(entries, position.unwrap());
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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::pause_audio,
            commands::resume_audio,
            commands::next_track,
            commands::previous_track,
            commands::get_volume,
            commands::set_volume,
            commands::get_audio_length,
            commands::get_current_position,
            commands::get_current_music_index,
            commands::set_current_time,
            commands::get_playlist_len,
            commands::get_current_track_name,
            commands::shuffle_playlist,
            commands::test,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
