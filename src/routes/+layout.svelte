<script>
	import '../app.css';
	let { children } = $props();
</script>

<main data-tauri-drag-region class="bg-black-500 h-screen w-screen overflow-hidden rounded-2xl p-4">
	{@render children()}
</main>
