import { invoke } from '@tauri-apps/api/core';

export function set_volume(value) {
	invoke('set_volume', { volume: value / 300 });
}

export const get_volume = () => invoke('get_volume');
export const get_current_track_name = () => invoke('get_current_track_name');
export const get_playlist_len = () => invoke('get_playlist_len');
export const get_audio_length = async () =>
	invoke('get_audio_length', { musicIndex: await get_current_music_index() });
export const get_current_music_index = () => invoke('get_current_music_index');
