<script lang="ts">
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
	let selectedMonth: string | null = null;
	let selectedDay: number = 1;
	$: selected = selectedMonth !== null ? `${selectedMonth} ${selectedDay}` : 'today';
	export let selectedDates: string[] = [];
	function select() {
		selectedDates = [selected, ...selectedDates];
		selectedMonth = null;
		selectedDay = 1;
	}
</script>

<div>
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
				class="w-10 text-center"
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

		<button
			on:click={select}
			class="btn btn-primary"
			class:btn-disabled={selectedMonth == null || selectedDates.includes(selected)}>Add</button
		>
	</div>

	{#if selectedDates.length > 0}
		<ul id="date-list" class="flex flex-col p-2 border-2 rounded-box w-fit">
			{#each selectedDates as date, i}
				<!-- TODO: argh where do we put the trash can icon -->
				<li
					class="my-1 btn btn-accent gap-1"
					on:click={() => {
						selectedDates = selectedDates.slice(0, i).concat(selectedDates.slice(i + 1));
					}}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						viewBox="0 0 20 20"
						fill="currentColor"
					>
						<path
							fill-rule="evenodd"
							d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
							clip-rule="evenodd"
						/>
					</svg>
					<span>{date}</span>
				</li>
			{/each}
		</ul>
	{:else}
		<p
			class="text-red-500 text-center font-bold border-2 dark:bg-neutral p-2 py-5 rounded-box w-fit"
		>
			No dates selected
		</p>
	{/if}
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
