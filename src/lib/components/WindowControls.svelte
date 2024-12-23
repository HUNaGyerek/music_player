<script>
	import { Minus, Settings, X } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	function handleSettings() {}
	async function handleTray() {
		await getCurrentWindow().minimize();
	}
	async function handleExit() {
		await getCurrentWindow().close();
	}
</script>

<div>
	<button onclick={handleSettings}><Settings /></button>
	<button onclick={handleTray}><Minus /></button>
	<button onclick={handleExit}><X /></button>
</div>
