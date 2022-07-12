<script lang="ts">
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

<p>What if I...</p>

<p>Current grade: {currentGrade}</p>
<p>Calculated grade: {newScore}</p>

<ul class="mx-auto menu shadow-lg w-56 p-2 rounded-box max-h-md overflow-y-scroll">
	{#each artificialAssignments as score, i}
		<li>
			<span
				on:click={() => {
					artificialAssignments.splice(i, 1);
					artificialAssignments = artificialAssignments;
				}}>{score}</span
			>
		</li>
	{:else}
		<li><span>Please enter some assignments</span></li>
	{/each}
</ul>
<h2>Add assignment</h2>
<div>
	<input type="number" min="0" bind:value={got} />/<input
		type="number"
		min="1"
		bind:value={total}
	/>
</div>
<button
	class="btn btn-primary"
	on:click={() => {
		artificialAssignments = [...artificialAssignments, (got / total) * 100];
	}}>Add</button
>
