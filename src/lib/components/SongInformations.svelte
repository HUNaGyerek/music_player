<script>
	import { get_audio_length, get_current_track_name } from '$lib/invoke';
	import { convertSecondsToMinuteText } from '$lib/utils';
	import { onMount } from 'svelte';

	let title = $state();
	let author = $state();
	let lenght = $state();
	onMount(async () => {
		title = await get_current_track_name();
		lenght = convertSecondsToMinuteText(await get_audio_length());
	});
</script>

<div data-tauri-drag-region class="flex select-none gap-4">
	<img class="pointer-events-none h-full rounded-xl" src="https://placehold.co/80" alt="" />
	<div class="pointer-events-none flex flex-col text-sm text-white">
		<h2 class="text-base font-bold">{title || 'asd'}</h2>
		<p>Author: {author || ''}</p>
		<p>{lenght || '00:00:00'}</p>
	</div>
</div>
