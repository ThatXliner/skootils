<script lang="ts">
	export let categories: string[];
	export let unweighted = false;
	export let artificialAssignments: { type: string; score: { recieved: number; total: number } }[];
	let recieved = 0;
	$: if (recieved < 0) {
		recieved = 0;
	}
	let total = 5;
	$: if (total < 0) {
		total = 0;
	}
	let selectedCategory: string;
</script>

<div class="bg-base-300 rounded-box p-3 max-w-fit mx-auto">
	<div>
		<h2 class="text-lg font-bold mb-1">Add assignment</h2>
		<input
			class="rounded bg-base-200 w-20 p-2 text-lg"
			type="number"
			min="0"
			bind:value={recieved}
		/>
		<span class="font-bold">out of</span>
		<input class="rounded bg-base-200 w-20 p-2 text-lg" type="number" min="0" bind:value={total} />
		<button
			class="mx-2 float-right btn btn-primary"
			disabled={selectedCategory == 'Pick one'}
			on:click={() => {
				artificialAssignments = [
					...artificialAssignments,
					{ type: selectedCategory, score: { recieved, total } }
				];
				selectedCategory = unweighted ? 'Unweighted' : 'Pick one';
			}}>Add</button
		>
	</div>
	<select class="select select-bordered w-full mt-3" bind:value={selectedCategory}>
		{#if unweighted}
			<option disabled selected>Unweighted</option>
		{:else}
			<option disabled selected>Pick one</option>
			{#each categories as category}
				<option>{category}</option>
			{/each}{/if}
	</select>
</div>
