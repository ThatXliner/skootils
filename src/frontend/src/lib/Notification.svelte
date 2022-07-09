<script lang="ts">
	import { fade } from 'svelte/transition';
	export let notification: { text: string; type: 'info' | 'success' | 'warning' | 'error' };
	// `onRemove` function will be passed into your component.
	export let onRemove: () => void;
</script>

<div class="alert alert-success shadow-lg m-1 p-2" out:fade>
	<div>{notification.text}</div>
	<button
		class="btn btn-ghost"
		on:click={() => {
			onRemove();
		}}>&times;</button
	>
</div>
