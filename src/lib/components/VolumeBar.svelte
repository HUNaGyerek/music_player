<script>
	import { get_volume, set_volume } from '$lib/invoke';
	import { Volume, Volume1, Volume2, VolumeX } from 'lucide-svelte';
	import { onMount } from 'svelte';

	function handleChangeInput() {
		if (isMuted) {
			isMuted = false;
		}
		set_volume(volume);
	}

	let isMuted = $state(false);
	let lastVolume = $state(0);
	function toggleMuted() {
		isMuted = !isMuted;

		if (isMuted) {
			lastVolume = volume;
			volume = 0;
			set_volume(0);
		} else {
			volume = lastVolume;
			set_volume(lastVolume);
		}
	}

	let volume = $state(0);
	$effect(async () => {
		document.querySelector('#volume-bar').style.setProperty('--value', `${volume}%`);
	});

	onMount(async () => {
		volume = await get_volume();
	});
</script>

<div class="flex gap-2">
	<button onclick={toggleMuted}>
		{#if volume == 0 || isMuted}
			<VolumeX strokeWidth={3} fill="white" />
		{:else if Math.ceil(volume / 33) == 1}
			<Volume strokeWidth={3} fill="white" />
		{:else if Math.ceil(volume / 33) == 2}
			<Volume1 strokeWidth={3} fill="white" />
		{:else}
			<Volume2 strokeWidth={3} fill="white" />
		{/if}
	</button>
	<input
		id="volume-bar"
		class="slider w-20"
		type="range"
		oninput={handleChangeInput}
		min="0"
		max="100"
		bind:value={volume}
	/>
</div>

<style lang="postcss">
	.slider {
		@apply cursor-pointer appearance-none bg-transparent;
		@apply focus:outline-none;
	}
	.slider::-webkit-slider-thumb {
		@apply -mt-1 h-3 w-3 appearance-none rounded-full bg-gray-400;
	}
	.slider::-webkit-slider-runnable-track {
		@apply h-1 rounded-full;
		@apply bg-gradient-to-r from-blue-500 from-[calc(var(--value))] to-gray-300 to-[calc(var(--value))];
	}
</style>
