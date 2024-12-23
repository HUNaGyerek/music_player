import { invoke } from '@tauri-apps/api/core';

export function set_volume(value) {
	invoke('set_volume', { volume: value / 300 });
}

export function get_volume() {
	return invoke('get_volume');
}
