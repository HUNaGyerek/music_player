<script>
	import { onMount } from 'svelte';
	import ControlInterface from '$lib/components/ControlInterface.svelte';
	import ProgressBar from '$lib/components/ProgressBar.svelte';
	import SongInformations from '$lib/components/SongInformations.svelte';
	import WindowControls from '$lib/components/WindowControls.svelte';
	import { disableContextMenu } from '$lib/utils';

	onMount(() => {
		disableContextMenu();
	});
</script>

<section>
	<!-- Window Controls -->
	<div class="absolute right-4 flex gap-1 text-white">
		<WindowControls />
	</div>

	<!-- Song Informations -->
	<div>
		<SongInformations />
	</div>

	<!-- ProgressBar -->
	<div class="mt-4 text-center">
		<ProgressBar />
	</div>

	<!-- Song Control Interfaces -->
	<div class="mt-1 text-white">
		<ControlInterface />
	</div>
</section>
