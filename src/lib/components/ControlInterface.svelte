<script>
	import { ListMusic, Shuffle, Pause, Play, SkipBack, SkipForward, ListX, X } from 'lucide-svelte';
	import VolumeBar from '$lib/components/VolumeBar.svelte';
	import {
		get_current_track_informations,
		next_track,
		pause_audio,
		previous_track,
		resume_audio,
		shuffle_playlist,
		toggle_playlist_menu
	} from '$lib/invoke';
	import { currentTrack, isMenuOpened, isShuffled } from '$lib/store';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';

	let isPlaying = $state(true);
	function togglePlaying() {
		isPlaying = !isPlaying;
	}

	function toggleMenu() {
		$isMenuOpened = !$isMenuOpened;
		toggle_playlist_menu();
	}

	function toggleShuffle() {
		$isShuffled = !$isShuffled;

		shuffle_playlist();
	}

	function handlePause() {
		pause_audio();
		togglePlaying();
	}

	function handleResume() {
		resume_audio();
		togglePlaying();
	}

	async function handlePrevious() {
		await previous_track();
		$currentTrack = await get_current_track_informations();
	}

	async function handleNext() {
		await next_track();
		$currentTrack = await get_current_track_informations();
	}
</script>

<div class="flex">
	<div class="flex w-full justify-start gap-2 ps-10">
		{#if !$isMenuOpened}
			<button onclick={toggleMenu}><ListMusic class="size-6" strokeWidth={2} fill="white" /></button
			>
		{:else}
			<button onclick={toggleMenu}><ListX class="size-6" strokeWidth={2} fill="white" /></button>
		{/if}
		<button onclick={toggleShuffle} class="relative flex flex-col justify-center">
			<Shuffle class="size-5 {$isShuffled ? 'absolute left-4 scale-[60%]' : ''}" strokeWidth={3} />
			{#if $isShuffled}
				<X class="absolute" strokeWidth={3} />
			{/if}
		</button>
	</div>
	<div class="flex w-full justify-center gap-2">
		<button onclick={handlePrevious}><SkipBack strokeWidth={3} fill="white" /></button>
		{#if isPlaying}
			<button onclick={handlePause}><Pause class="size-5" strokeWidth={3} fill="white" /></button>
		{:else}
			<button onclick={handleResume}><Play class="size-5" strokeWidth={3} fill="white" /></button>
		{/if}
		<button onclick={handleNext}><SkipForward class="size-5" strokeWidth={3} fill="white" /></button
		>
	</div>
	<div class="flex w-full justify-end pe-10">
		<VolumeBar />
	</div>
</div>
