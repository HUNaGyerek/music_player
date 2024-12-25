// import { writable, persisted } from 'svelte-persisted-store';
import { writable } from 'svelte/store';

export const currentTrack = writable();
// TODO: to be persisted on F5 but not on app close
export const isShuffled = writable(false);
export const isMenuOpened = writable(false);
