<script>
	import { get_current_track_informations } from '$lib/invoke';
	import { currentTrack } from '$lib/store';
	import { convertSecondsToMinuteText } from '$lib/utils';
	import { onMount } from 'svelte';

	onMount(async () => {
		$currentTrack = await get_current_track_informations();
	});
</script>

{#if $currentTrack}
	<div
		data-tauri-drag-region
		class="flex select-none {$currentTrack.thumbnail !== null ? 'gap-4' : ''}"
	>
		<div class="h-24">
			{#if $currentTrack.thumbnail}
				<img
					class="pointer-events-none h-full w-24 rounded-xl object-cover object-center"
					src="data:image/png;base64,{$currentTrack.thumbnail}"
					alt=""
				/>
			{/if}
		</div>
		<div class="pointer-events-none flex flex-col text-sm text-white">
			<h2 class="text-base font-bold">{$currentTrack.title || 'Not loaded'}</h2>
			<p>Author: {$currentTrack.author || ''}</p>
			<p>{convertSecondsToMinuteText($currentTrack.lenght) || '00:00:00'}</p>
		</div>
	</div>
{/if}
