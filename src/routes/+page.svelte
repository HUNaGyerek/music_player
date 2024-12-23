<script>
	import ProgressBar from '$lib/components/ProgressBar.svelte';
	import VolumeBar from '$lib/components/VolumeBar.svelte';
	import WindowControls from '$lib/components/WindowControls.svelte';
	import { ListMusic, Shuffle, Pause, Play, SkipBack, SkipForward } from 'lucide-svelte';

	let isPlaying = $state();
	function togglePlaying() {
		isPlaying = !isPlaying;
	}

	function handlePause() {
		togglePlaying();
	}

	function handleResume() {
		togglePlaying();
	}
</script>

<section>
	<!-- Window Controls -->
	<div class="absolute right-4 flex gap-1 text-white">
		<WindowControls />
	</div>

	<!-- Song Informations -->
	<div data-tauri-drag-region class="flex select-none gap-4">
		<img class="pointer-events-none h-full rounded-xl" src="https://placehold.co/80" alt="" />
		<div class="pointer-events-none flex flex-col text-sm text-white">
			<h2 class="text-base font-bold">asd</h2>
			<p>Author:</p>
			<p>00:00:00</p>
		</div>
	</div>

	<div class="mt-4 text-center">
		<ProgressBar />
	</div>

	<div class="mt-1 flex text-white">
		<div class="flex w-full justify-start gap-2 ps-10">
			<button><ListMusic class="size-5" strokeWidth={3} fill="white" /></button>
			<button><Shuffle class="size-5" strokeWidth={3} fill="white" /></button>
		</div>
		<div class="flex w-full justify-center gap-2">
			<button><SkipBack strokeWidth={3} fill="white" /></button>
			{#if isPlaying}
				<button onclick={handlePause}><Pause class="size-5" strokeWidth={3} fill="white" /></button>
			{:else}
				<button onclick={handleResume}><Play class="size-5" strokeWidth={3} fill="white" /></button>
			{/if}
			<button><SkipForward class="size-5" strokeWidth={3} fill="white" /></button>
		</div>
		<div class="flex w-full justify-end pe-10">
			<VolumeBar />
		</div>
	</div>
</section>
