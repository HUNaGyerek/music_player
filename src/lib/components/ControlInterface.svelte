<script>
	import { ListMusic, Shuffle, Pause, Play, SkipBack, SkipForward } from 'lucide-svelte';
	import VolumeBar from '$lib/components/VolumeBar.svelte';

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

<div class="flex">
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
