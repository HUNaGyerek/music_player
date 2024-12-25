<script>
	import '../app.css';
	import Playlist from '$lib/components/Playlist.svelte';
	import { isMenuOpened } from '$lib/store';

	let { children } = $props();
</script>

<section class="flex h-screen flex-col overflow-x-hidden [&::-webkit-scrollbar]:hidden">
	<main data-tauri-drag-region class="z-50 h-[204px] w-screen rounded-2xl bg-black-500 p-4">
		{@render children()}
	</main>
	<div
		class="{$isMenuOpened
			? 'flex'
			: 'hidden'} -mt-6 h-[calc(100vh-204px+24px)] w-screen flex-col rounded-2xl bg-black-400 px-4 pb-4 pt-6"
	>
		<Playlist />
	</div>
</section>
