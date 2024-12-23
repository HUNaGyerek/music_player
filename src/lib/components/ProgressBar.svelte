<script>
	let progress = $state();
	$effect(() => {
		console.log(`Progress: ${progress}`);
		document.querySelector('#progress-bar').style.setProperty('--value', `${progress}%`);
	});
</script>

<div>
	<input id="progress-bar" class="slider w-full" type="range" bind:value={progress} />
	<p class="select-none text-sm text-white">00:00:00</p>
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
