<script lang="ts">
	import AlertError from './AlertError.svelte';
	import WeightPicker from './WeightPicker.svelte';
	export let currentScore: number;
	export let assignments: { percent?: number; type: string }[];
	const categories: string[] = [...new Set(assignments.map((t) => t.type))];

	let weights: { [key: string]: number } = {};
	let artificialAssignments: [string, number][] = [];
	$: realAssignments = assignments
		.filter((x) => x.percent !== null)
		.map((x) => [x.type, x.percent!]);
	let got = 0;
	$: if (got < 0) {
		got = 0;
	}

	let total = 5;
	function sum(x: number[]): number {
		let output = 0;
		for (let item of x) {
			output += item;
		}
		return output;
	}
	function weightedAverage(
		assignments: [string, number][],
		weights: { [key: string]: number }
	): number {
		let averages: { [key: string]: number[] } = Object.fromEntries(categories.map((e) => [e, []]));
		assignments.forEach((e) => {
			averages[e[0]].push(e[1]);
		});

		return (
			sum(Object.entries(averages).map((e) => (weights[e[0]] * sum(e[1])) / e[1].length)) /
			sum(Object.values(weights))
		);
	}
	let selectedCategory: string;
	// todo: new equation
	$: newScore = weightedAverage(realAssignments.concat(artificialAssignments), weights);
</script>

<div class="space-y-3">
	<div class="flex justify-around text-black">
		<span class="p-3 bg-red-300 rounded-box">Current grade: <b>{currentScore}%</b></span>
		<span class="p-3 bg-blue-300 rounded-box">
			<!-- 2 decimal points -->
			Calculated grade:
			<b>{parseFloat(newScore.toPrecision(4))}%</b>
		</span>
	</div>

	<WeightPicker {categories} bind:weights />
	{#if artificialAssignments.length > 0}
		<div
			class="bg-base-200 mx-auto shadow-lg p-2 rounded-box max-h-40 overflow-y-auto flex flex-wrap w-full"
		>
			{#each artificialAssignments as [type, score], i}
				<span
					class="bg-info shadow-md p-2 m-1 rounded w-fit"
					on:click={() => {
						artificialAssignments = artificialAssignments.filter((_, index) => index != i);
					}}
					>{score}%<svg
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
				<path
					fill-rule="evenodd"
					d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
					clip-rule="evenodd"
				/>
			{/each}
		</div>
	{:else}
		<AlertError message="Please enter some theoretical scores" extraClasses="mx-auto" />
	{/if}
</div>

<div class="bg-base-300 rounded-box p-3 w-fit mx-auto">
	<h2 class="text-lg font-bold mb-1">Add assignment</h2>
	<input class="rounded bg-base-200 w-20 p-2 text-lg" type="number" min="0" bind:value={got} />
	<span class="font-bold">out of</span>
	<input class="rounded bg-base-200 w-20 p-2 text-lg" type="number" min="1" bind:value={total} />
	<button
		class="mx-2 float-right btn btn-primary"
		on:click={() => {
			artificialAssignments = [...artificialAssignments, (got / total) * 100];
		}}>Add</button
	>
</div>
<button
	class="mx-auto mt-3 btn btn-error"
	on:click={() => {
		artificialAssignments = [];
	}}>Clear all</button
>
