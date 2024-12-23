<script>
	import { ListMusic, Shuffle, Pause, Play, SkipBack, SkipForward } from 'lucide-svelte';
	import VolumeBar from '$lib/components/VolumeBar.svelte';
	import {
		get_current_track_informations,
		next_track,
		pause_audio,
		previous_track,
		resume_audio
	} from '$lib/invoke';
	import { currentTrack } from '$lib/store';

	let isPlaying = $state(true);
	function togglePlaying() {
		isPlaying = !isPlaying;
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
		<button><ListMusic class="size-5" strokeWidth={3} fill="white" /></button>
		<button><Shuffle class="size-5" strokeWidth={3} fill="white" /></button>
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
