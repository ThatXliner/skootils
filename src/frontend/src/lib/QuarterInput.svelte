<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	let _selQ: { [key: string]: boolean } = {};
	$: realSel = Array.from(Object.entries(_selQ))
		.filter((x) => x[1])
		.map((x) => x[0]);
	export let quarterChoices: string[];
	let _latest = true;
</script>

<div class="form-control">
	<label class="label cursor-pointer">
		<span class="label-text text-2xl mx-2">Latest quarter</span>
		<input type="checkbox" class="toggle toggle-primary" bind:checked={_latest} />
	</label>
</div>
{#if !_latest}
	<div class="form-control mx-auto p-2 bg-base-100 rounded-box">
		{#each quarterChoices as quarter}
			<label class="label cursor-pointer flex">
				<span class="label-text text-lg">{quarter}</span>
				<input
					type="checkbox"
					class="checkbox"
					on:click={() => {
						_selQ[quarter] = !_selQ[quarter];
					}}
				/>
			</label>
		{/each}
	</div>
{/if}

<button
	class="btn btn-primary mt-2"
	class:btn-disabled={!_latest && realSel.length == 0}
	on:click={() => {
		if (_latest) {
			dispatch('submit', null);
		} else {
			dispatch('submit', realSel.sort());
		}
	}}>Go!</button
>
