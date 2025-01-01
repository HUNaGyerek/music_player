#![allow(dead_code)]

use base64::Engine;
use id3::Tag;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle, Sink};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::utils::ShuffleState;

use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

const DEFAULT_VALUE: f32 = 0.01;

#[derive(Default)]
pub struct AudioPlayer {
    stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sink: Option<Arc<Mutex<Sink>>>,
    volume: f32,
    start_time: Option<Instant>,
    paused_position: Option<Duration>,
    playlist: Vec<PathBuf>,
    current_index: usize,
    track_duration: Option<Duration>,
    shuffle_indecies: Vec<usize>,
    shuffle_state: ShuffleState,
}

#[derive(Serialize, Deserialize)]
pub struct Music {
    pub title: String,
    pub lenght: u64,
    pub thumbnail: Option<String>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.set_volume(DEFAULT_VALUE);
        Self {
            stream: Some(stream),
            stream_handle: Some(stream_handle),
            sink: Some(Arc::new(Mutex::new(sink))),
            volume: DEFAULT_VALUE,
            start_time: None,
            paused_position: None,
            playlist: Vec::new(),
            current_index: 0,
            track_duration: None,
            shuffle_indecies: Vec::new(),
            shuffle_state: ShuffleState::Unshuffled,
        }
    }

    pub fn load_playlist(&mut self, playlist: Vec<PathBuf>, index_of_opened_track: usize) {
        self.playlist = playlist;
        self.current_index = index_of_opened_track;
        self.play();
    }

    pub fn get_playlist(&self) -> Vec<Music> {
        self.playlist
            .iter()
            .enumerate()
            .map(|(idx, path)| {
                let title = path.file_stem().unwrap().to_str().unwrap().to_string();
                let lenght = self.get_lenght_by_index(idx).unwrap();
                let thumbnail = None;
                Music {
                    title,
                    lenght,
                    thumbnail,
                }
            })
            .collect()
    }

    pub fn play_by_index(&mut self, idx: usize) {
        if idx < self.playlist.len() {
            self.current_index = idx;
            self.play(); // Play the track at the specified index
        }
    }

    fn play(&mut self) {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().stop();

            let idx = self.get_current_index();
            if idx < self.playlist.len() {
                let file_path = &self.playlist[idx];
                let file = File::open(file_path).unwrap();
                let source = Decoder::new(BufReader::new(file));

                if let Ok(source) = source {
                    self.track_duration = Some(source.total_duration().unwrap_or(Duration::ZERO));
                    sink.lock().unwrap().append(source);

                    self.start_time = Some(Instant::now());
                    self.paused_position = None;
                } else {
                    self.playlist.remove(idx); // remove the file from the list
                    self.current_index -= 1; // decrement to play the same index that will be the next
                    self.next();
                }
            }
        }
    }

    pub fn next(&mut self) {
        if let ShuffleState::Shuffled = self.shuffle_state {
            if !self.shuffle_indecies.contains(&self.current_index) {
                self.shuffle_indecies.push(self.current_index)
            }
            self.current_index = self.shuffle();
            self.play();
        } else {
            if self.get_current_index() + 1 < self.playlist.len() {
                self.current_index += 1;
                self.play();
            }
        }
    }

    pub fn previous(&mut self) {
        if let ShuffleState::Shuffled = self.shuffle_state {
            if self.shuffle_indecies.is_empty() {
                self.current_index = self.shuffle();
            } else {
                self.current_index = *self.shuffle_indecies.last().unwrap();
                self.shuffle_indecies.pop();
            }
            self.play();
        } else {
            if self.get_current_index() > 0 {
                self.current_index -= 1;
                self.play();
            }
        }
    }

    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().pause();

            if let Some(start_time) = self.start_time {
                self.paused_position = Some(start_time.elapsed());
            }
        }
    }

    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().play();

            if let Some(paused_position) = self.paused_position {
                self.start_time = Some(Instant::now() - paused_position);
                self.paused_position = None;
            }
        }
    }

    pub fn get_volume(&self) -> Option<f32> {
        Some(self.volume * 300f32)
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Some(sink) = &self.sink {
            self.volume = volume;
            sink.lock().unwrap().set_volume(volume);
        }
    }

    pub fn get_lenght_by_index(&self, audio_index: usize) -> Option<u64> {
        if audio_index < self.playlist.len() {
            let src = File::open(self.playlist[audio_index].clone()).unwrap();
            let mss = MediaSourceStream::new(Box::new(src), Default::default());
            let hint = Hint::new();

            let probed = get_probe().format(
                &hint,
                mss,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            );

            if let Ok(probed) = probed {
                let format = probed.format;

                let duration = format
                    .default_track()
                    .unwrap()
                    .codec_params
                    .time_base
                    .map(|time_base| {
                        time_base.calc_time(
                            format
                                .default_track()
                                .unwrap()
                                .codec_params
                                .n_frames
                                .unwrap(),
                        )
                    })
                    .unwrap();

                return Some(duration.seconds);
            }
        }
        Some(1)
    }

    pub fn get_length(&self) -> Option<u64> {
        let audio_index = self.get_current_index();
        if audio_index < self.playlist.len() {
            let src = File::open(self.playlist[audio_index].clone()).unwrap();
            let mss = MediaSourceStream::new(Box::new(src), Default::default());
            let hint = Hint::new();

            let probed = get_probe().format(
                &hint,
                mss,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            );

            if let Ok(probed) = probed {
                let format = probed.format;

                let duration = format
                    .default_track()
                    .unwrap()
                    .codec_params
                    .time_base
                    .map(|time_base| {
                        time_base.calc_time(
                            format
                                .default_track()
                                .unwrap()
                                .codec_params
                                .n_frames
                                .unwrap(),
                        )
                    })
                    .unwrap();

                return Some(duration.seconds);
            }
        }
        Some(1)
    }

    pub fn get_position(&self) -> Option<u64> {
        if let Some(start_time) = self.start_time {
            if let Some(paused_position) = self.paused_position {
                return Some(paused_position.as_secs());
            } else {
                return Some(start_time.elapsed().as_secs());
            }
        }
        None
    }

    pub fn get_current_index(&self) -> usize {
        self.current_index
    }

    pub fn set_current_time(&mut self, position: u64) {
        if let Some(sink) = &self.sink {
            if self.get_current_index() < self.playlist.len() {
                sink.lock().unwrap().stop();

                let file_path = &self.playlist[self.get_current_index()];
                let file = File::open(file_path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                let source = source.skip_duration(Duration::from_secs(position));
                sink.lock().unwrap().append(source);

                self.start_time = Some(Instant::now() - Duration::from_secs(position));
                self.paused_position = None;
            }
        }
    }

    pub fn get_playlist_len(&self) -> usize {
        self.playlist.len()
    }

    pub fn get_music_name(&self) -> String {
        self.playlist[self.get_current_index()]
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn toggle_shuffle(&mut self) {
        match self.shuffle_state {
            ShuffleState::Unshuffled => {
                println!("{:?}", "shuff");
                self.shuffle_state = ShuffleState::Shuffled;
            }
            ShuffleState::Shuffled => {
                println!("{:?}", "unshuff");
                self.shuffle_state = ShuffleState::Unshuffled;
            }
        }
    }

    fn shuffle(&self) -> usize {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.playlist.len());
        if idx == self.playlist.len() - 1 {
            return idx - 1;
        } else {
            return idx + 1;
        }
    }

    pub fn get_thumbnail(&self) -> Option<String> {
        match Tag::read_from_path(&self.playlist[self.get_current_index()]) {
            Ok(tag) => {
                if let Some(picture) = tag.pictures().next() {
                    // Encode the picture data as Base64
                    let base64_data =
                        base64::engine::general_purpose::STANDARD.encode(&picture.data);
                    Some(base64_data)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

unsafe impl Send for AudioPlayer {}
