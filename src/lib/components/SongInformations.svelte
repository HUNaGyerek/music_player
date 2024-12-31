<script>
	import { get_current_track_informations } from '$lib/invoke';
	import { currentTrack } from '$lib/store';
	import { convertSecondsToMinuteText } from '$lib/utils';
	import { onMount } from 'svelte';

	onMount(async () => {
		$currentTrack = await get_current_track_informations();
	});
</script>

<div data-tauri-drag-region class="flex select-none gap-4">
	{#if $currentTrack}
		<img
			class="pointer-events-none h-24 w-24 rounded-xl object-cover object-center"
			src="data:image/png;base64,{$currentTrack.thumbnail}"
			alt=""
		/>
		<div class="pointer-events-none flex flex-col text-sm text-white">
			<h2 class="text-base font-bold">{$currentTrack.title || 'Not loaded'}</h2>
			<p>Author: {$currentTrack.author || ''}</p>
			<p>{convertSecondsToMinuteText($currentTrack.lenght) || '00:00:00'}</p>
		</div>
	{/if}
</div>
