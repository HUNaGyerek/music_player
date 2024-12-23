<script>
	// import { get_current_track_informations } from '$lib/invoke';
	import { currentTrack } from '$lib/store';
	import { convertSecondsToMinuteText } from '$lib/utils';

	$effect(async () => {
		console.log($currentTrack);
	});
</script>

<div data-tauri-drag-region class="flex select-none gap-4">
	<img class="pointer-events-none h-full rounded-xl" src="https://placehold.co/80" alt="" />
	<div class="pointer-events-none flex flex-col text-sm text-white">
		<h2 class="text-base font-bold">{$currentTrack.title || 'asd'}</h2>
		<p>Author: {$currentTrack.author || ''}</p>
		<p>{convertSecondsToMinuteText($currentTrack.lenght) || '00:00:00'}</p>
	</div>
</div>
