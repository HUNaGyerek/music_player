import { invoke } from '@tauri-apps/api/core';

export function set_volume(value) {
	invoke('set_volume', { volume: value / 300 });
}

export function set_current_time(value) {
	invoke('set_current_time', { position: value });
}

export const play_by_index = (idx) => invoke('play_by_index', { idx });
export const pause_audio = () => invoke('pause_audio');
export const resume_audio = () => invoke('resume_audio');
export const previous_track = async () => await invoke('previous_track');
export const next_track = async () => await invoke('next_track');
export const get_volume = () => invoke('get_volume');
export const get_current_track_name = () => invoke('get_current_track_name');
export const get_current_audio_length = () => invoke('get_current_audio_length');
export const get_playlist_len = () => invoke('get_playlist_len');
export const get_current_music_index = () => invoke('get_current_music_index');
export const get_current_track_informations = () => invoke('get_current_track_informations');
export const get_current_position = () => invoke('get_current_position');
export const shuffle_playlist = () => invoke('shuffle_playlist');
export const toggle_playlist_menu = () => invoke('toggle_playlist_menu');
export const get_playlist = () => invoke('get_playlist');
