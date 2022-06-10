<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	const MONTHS = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	];
	// let _selected: string = new Date().toISOString().slice(0, 10);
	let selectedMonth: string | null = null;
	let selectedDay: number = 1;
	function select() {
		dispatch('select', `${selectedMonth} ${selectedDay}`);
		selectedMonth = null;
		selectedDay = 1;
	}
</script>

<div class="flex space-x-2 my-2">
	<div class="dropdown">
		{#if selectedMonth == null}
			<label for="month-select" tabindex="0" class="btn">Select a month</label>
		{:else}
			<label for="month-select" tabindex="0" class="btn btn-accent">{selectedMonth}</label>
		{/if}
		<ul
			id="month-select"
			tabindex="0"
			class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52 max-h-52 overflow-y-scroll"
		>
			{#each MONTHS as month}
				<li
					class:active={month == selectedMonth}
					on:click={() => {
						selectedMonth = month;
					}}
				>
					<span>{month}</span>
				</li>
			{/each}
		</ul>
	</div>
	<div class="btn-group">
		<button
			class="btn text-lg"
			on:click={() => {
				selectedDay--;
			}}>-</button
		>
		<input
			type="number"
			class="w-10 p-2"
			bind:value={selectedDay}
			on:blur={() => {
				if (selectedDay < 1) {
					selectedDay = 1;
				}
				if (selectedDay > 31) {
					selectedDay = 31;
				}
			}}
		/>

		<button
			class="btn text-lg"
			on:click={() => {
				selectedDay++;
			}}>+</button
		>
	</div>

	<button on:click={select} class="btn btn-primary" class:btn-disabled={selectedMonth == null}
		>Add</button
	>
</div>

<style>
	/* Chrome, Safari, Edge, Opera */
	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	/* Firefox */
	input[type='number'] {
		-moz-appearance: textfield;
	}
</style>
