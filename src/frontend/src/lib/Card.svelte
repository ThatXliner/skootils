<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let title: string;
	export let summary: string;
	export let action: string = 'Go!';
	export let href: string | null = null;
	const dispatch = createEventDispatcher();
</script>

<div class="m-1 card w-80 h-fit bg-base-100 shadow-xl">
	<div class="card-body">
		<h2 class="card-title">{title}</h2>
		<p>{summary}</p>
		<div class="card-actions justify-end">
			{#if href == null}
				<button
					class="btn btn-primary"
					on:click={(event) => {
						dispatch('click', event.detail);
					}}>{action}</button
				>
			{:else}
				<a class="btn btn-primary" {href}>{action}</a>
			{/if}
		</div>
	</div>
</div>
