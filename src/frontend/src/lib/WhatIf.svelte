<script lang="ts">
	import AlertError from './AlertError.svelte';

	export let currentScore: number;
	export let assignmentCount: number;
	let artificialAssignments: number[] = [];
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
	$: currentGrade = currentScore;
	$: newScore =
		(currentGrade * assignmentCount + sum(artificialAssignments)) /
		(assignmentCount + artificialAssignments.length);
</script>

<!-- <div class="flex space-y-4 flex-col"> -->
<!-- <p class="p-5 bg-amber-300 rounded-box">What if I...</p> -->
<div class="flex justify-around">
	<span class="p-3 bg-red-300 rounded-box">Current grade: <b>{currentGrade}%</b></span>
	<span class="p-3 bg-blue-300 rounded-box">
		<!-- 2 decimal points -->
		Calculated grade: <b>{parseFloat(newScore.toPrecision(4))}%</b>
	</span>
</div>

<div class="my-3">
	{#if artificialAssignments.length > 0}
		<div
			class="bg-base-200 mx-auto shadow-lg p-2 rounded-box max-h-40 overflow-y-auto flex flex-wrap w-full"
		>
			{#each artificialAssignments as score, i}
				<span
					class="bg-info shadow-md p-2 m-1 rounded w-fit"
					on:click={() => {
						artificialAssignments.splice(i, 1);
						artificialAssignments = artificialAssignments;
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

<!-- <div class="flex justify-center mx-auto">
	<div class="bg-base-300 p-3 w-fit ">
		<input class="rounded bg-base-200 w-20 p-3 text-lg" type="number" min="0" bind:value={got} />
		<span class="font-bold">out of</span>
		<input class="rounded bg-base-200 w-20 p-3 text-lg" type="number" min="1" bind:value={total} />
	</div>
	<button
		class="btn btn-primary h-full"
		on:click={() => {
			artificialAssignments = [...artificialAssignments, (got / total) * 100];
		}}>Add</button
	>
</div> -->
