<script lang="ts">
	import AlertError from './AlertError.svelte';
	import WeightPicker from './WeightPicker.svelte';
	export let currentScore: number;
	export let assignments: { score: { total: number; recieved?: number }; type: string }[];
	$: categories = [...new Set(assignments.map((t) => t.type))];

	let weights: { [key: string]: number } | null = null;
	let artificialAssignments: { score: { total: number; recieved: number }; type: string }[] = [];
	let got = 0;
	$: if (got < 0) {
		got = 0;
	}
	let total = 5;
	$: if (total < 0) {
		total = 0;
	}
	function sum(x: number[]): number {
		let output = 0;
		for (let item of x) {
			output += +item;
		}
		return output;
	}
	let selectedCategory: string;
	function calculateScore(
		assignments: { score: { recieved: number; total: number }; type: string }[],
		weights: { [key: string]: number } | null
	) {
		console.log(assignments);
		if (weights === null) {
			// unweighted
			return (
				sum(assignments.map((e) => e.score.recieved)) / sum(assignments.map((e) => e.score.total))
			);
		}
		let output = 0;
		for (let [category, weight] of Object.entries(weights)) {
			const totalAssignments = assignments.filter((e) => e.type === category);
			const totalEarned = sum(totalAssignments.map((e) => e.score.recieved));
			const totalPoints = sum(totalAssignments.map((e) => e.score.total));
			const categoryAverage = totalEarned / totalPoints || 1;
			console.log(category, categoryAverage);
			output += categoryAverage * weight;
		}
		return output;
	}
	// todo: semester support
	$: newScore =
		calculateScore(
			assignments.filter((e) => e.score.recieved !== null).concat(artificialAssignments),
			weights
		) * 100;
</script>

<div class="space-y-3">
	<div class="flex justify-around text-black">
		<span class="p-3 bg-red-300 rounded-box">Current grade: <b>{currentScore}%</b></span>
		<span class="p-3 bg-blue-300 rounded-box">
			<!-- 2 decimal points -->
			Calculated grade: <b>{parseFloat(newScore.toPrecision(4))}%</b>
		</span>
	</div>
	<WeightPicker {categories} bind:weights />
	{#if artificialAssignments.length > 0}
		<div
			class="bg-base-200 mx-auto shadow-lg p-2 rounded-box max-h-40 overflow-y-auto flex flex-wrap w-full"
		>
			{#each artificialAssignments as given, i}
				{@const type = given.type}
				{@const score = (given.score.recieved / (given.score.total || 1)) * 100}
				<span
					class="bg-info shadow-md p-2 m-1 rounded w-fit dark:text-black"
					on:click={() => {
						artificialAssignments = artificialAssignments.filter((_, index) => index != i);
					}}
					>{score}% ({type})<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5 inline"
						viewBox="0 0 20 20"
						fill="currentColor"
					>
						<path
							fill-rule="evenodd"
							d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
							clip-rule="evenodd"
						/>
					</svg></span
				>
			{/each}
		</div>
	{:else}
		<AlertError message="Please enter some theoretical scores" extraClasses="mx-auto" />
	{/if}

	<div class="bg-base-300 rounded-box p-3 max-w-fit mx-auto">
		<div>
			<h2 class="text-lg font-bold mb-1">Add assignment</h2>
			<input class="rounded bg-base-200 w-20 p-2 text-lg" type="number" min="0" bind:value={got} />
			<span class="font-bold">out of</span>
			<input
				class="rounded bg-base-200 w-20 p-2 text-lg"
				type="number"
				min="0"
				bind:value={total}
			/>
			<button
				class="mx-2 float-right btn btn-primary"
				disabled={selectedCategory == 'Pick one'}
				on:click={() => {
					artificialAssignments = [
						...artificialAssignments,
						{ type: selectedCategory, score: { recieved: got, total } }
					];
					selectedCategory = 'Pick one';
				}}>Add</button
			>
		</div>
		<select class="select select-bordered w-full mt-3" bind:value={selectedCategory}>
			<option disabled selected>Pick one</option>
			{#each categories as category}
				<option>{category}</option>
			{/each}
		</select>
	</div>
</div>
<button
	class="mx-auto mt-3 btn btn-error"
	disabled={artificialAssignments.length == 0}
	on:click={() => {
		artificialAssignments = [];
	}}>Clear all</button
>
