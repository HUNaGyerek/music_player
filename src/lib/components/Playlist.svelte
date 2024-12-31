<script>
	import { get_current_track_informations, get_playlist, play_by_index } from '$lib/invoke';
	import { currentTrack } from '$lib/store';
	import { convertSecondsToMinuteText } from '$lib/utils';
	import { Play } from 'lucide-svelte';

	async function handleChangeMusic(idx) {
		play_by_index(idx);
	}
</script>

<div class="scroll-container relative overflow-y-auto">
	<div class="me-2">
		{#await get_playlist() then playlist}
			{#each playlist as music, idx}
				<button
					ondblclick={() => handleChangeMusic(idx)}
					class="mt-2 flex h-auto w-full items-center rounded-lg px-4 py-2 text-xs font-semibold shadow-lg {$currentTrack.title ===
					music.title
						? 'bg-blue-400 text-white hover:bg-blue-500'
						: 'text-black bg-gray-200 hover:bg-gray-400'}"
				>
					<div class="flex w-full flex-col items-start">
						<h2 class="text-sm">{music.title}</h2>
						<p>Author</p>
					</div>

					<div class="flex items-center justify-end gap-2">
						<p>{convertSecondsToMinuteText(music.lenght)}</p>
						<Play strokeWidth={3} color={$currentTrack.title === music.title ? 'white' : 'black'} />
					</div>
				</button>
			{/each}
		{/await}
	</div>
</div>

<style lang="postcss">
	.scroll-container::-webkit-scrollbar {
		@apply w-1;
	}
	.scroll-container::-webkit-scrollbar-track {
		@apply mt-2 rounded-full bg-gray-200;
	}
	.scroll-container::-webkit-scrollbar-thumb {
		@apply rounded-full bg-black-600;
	}
</style>
