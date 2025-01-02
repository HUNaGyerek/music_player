<script>
	import {
		get_current_audio_length,
		get_current_music_index,
		get_current_track_informations,
		set_current_time
	} from '$lib/invoke';
	import { currentTrack } from '$lib/store';
	import { convertSecondsToMinuteText } from '$lib/utils';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	function seek_to(event) {
		set_current_time(+event.target.value);
	}

	let progress = $state(0);
	$effect(() => {
		const progressPercentage = (progress / lenght) * 100 || 0;

		// console.log(`Progress: ${progressPercentage}%`);
		document.querySelector('#progress-bar').style.setProperty('--value', `${progressPercentage}%`);
	});

	let lenght = $state();
	onMount(async () => {
		lenght = (await get_current_audio_length()) || 0;

		listen('track-progress', (event) => {
			progress = event.payload;
		});

		listen('track-changed', async (event) => {
			$currentTrack = await get_current_track_informations();
			lenght = $currentTrack.lenght;
		});
	});
</script>

<div>
	<input
		id="progress-bar"
		class="slider w-full"
		type="range"
		max={lenght}
		bind:value={progress}
		onchange={seek_to}
	/>
	<p class="select-none text-sm text-white">{convertSecondsToMinuteText(progress) || '00:00:00'}</p>
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
