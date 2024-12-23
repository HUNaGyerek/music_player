import { writable } from 'svelte/store';
import { get_current_track_informations } from './invoke';

export const currentTrack = writable(await get_current_track_informations());
