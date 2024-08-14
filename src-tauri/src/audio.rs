use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use symphonia::core::audio;

use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

const DEFAULT_VALUE: f32 = 0.1;

#[derive(Default)]
pub struct AudioPlayer {
    stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sink: Option<Arc<Mutex<Sink>>>,
    volume: f32,
    start_time: Option<Instant>,
    paused_position: Option<Duration>,
    playlist: Vec<String>,
    current_index: usize,
    track_duration: Option<Duration>,
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
        }
    }

    // pub fn initialize(&mut self) {
    //     let (stream, stream_handle) = OutputStream::try_default().unwrap();
    //     let sink = Sink::try_new(&stream_handle).unwrap();
    //     sink.set_volume(self.volume);
    //     self.stream = Some(stream);
    //     self.stream_handle = Some(stream_handle);
    //     self.sink = Some(Arc::new(Mutex::new(sink)));
    // }

    pub fn play_playlist(&mut self, playlist: Vec<String>) {
        self.playlist = playlist;
        self.current_index = 0;
        self.play_current_track();
    }

    pub fn play_current_track(&mut self) {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().stop();
            if self.current_index < self.playlist.len() {
                let file_path = &self.playlist[self.current_index];
                let file = File::open(file_path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                self.track_duration = Some(source.total_duration().unwrap_or(Duration::ZERO));
                sink.lock().unwrap().append(source);

                self.start_time = Some(Instant::now());
                self.paused_position = None;
            }
        }
    }

    pub fn next_track(&mut self) {
        if self.current_index + 1 < self.playlist.len() {
            self.current_index += 1;
            self.play_current_track();
        }
    }

    pub fn previous_track(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.play_current_track();
        }
    }

    pub fn pause_audio(&mut self) {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().pause();

            if let Some(start_time) = self.start_time {
                self.paused_position = Some(start_time.elapsed());
            }
        }
    }

    pub fn resume_audio(&mut self) {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().play();

            if let Some(paused_position) = self.paused_position {
                self.start_time = Some(Instant::now() - paused_position);
                self.paused_position = None;
            }
        }
    }

    pub fn get_volume(&self) -> Option<f32> {
        Some(self.volume * 100f32)
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Some(sink) = &self.sink {
            self.volume = volume;
            sink.lock().unwrap().set_volume(volume);
        }
    }

    pub fn get_audio_length(&self, audio_index: usize) -> Option<u64> {
        if audio_index < self.playlist.len() {
            let src = File::open(self.playlist[audio_index].clone()).unwrap();
            let mss = MediaSourceStream::new(Box::new(src), Default::default());
            let hint = Hint::new();

            let probed = get_probe()
                .format(
                    &hint,
                    mss,
                    &FormatOptions::default(),
                    &MetadataOptions::default(),
                )
                .unwrap();

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
        Some(1)
    }

    pub fn get_current_position(&self) -> Option<u64> {
        if let Some(start_time) = self.start_time {
            if let Some(paused_position) = self.paused_position {
                return Some(paused_position.as_secs());
            } else {
                return Some(start_time.elapsed().as_secs());
            }
        }
        None
    }

    pub fn get_current_music_index(&self) -> usize {
        self.current_index
    }

    pub fn set_current_time(&mut self, position: u64) {
        if let Some(sink) = &self.sink {
            if self.current_index < self.playlist.len() {
                sink.lock().unwrap().stop();

                let file_path = &self.playlist[self.current_index];
                let file = File::open(file_path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                let source = source.skip_duration(Duration::from_secs(position));
                sink.lock().unwrap().append(source);

                self.start_time = Some(Instant::now() - Duration::from_secs(position));
                self.paused_position = None;
            }
        }
    }
}

unsafe impl Send for AudioPlayer {}
