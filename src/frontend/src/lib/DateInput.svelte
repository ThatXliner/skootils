<script lang="ts">
	// TODO: Selection logic
	const monthFormatter = new Intl.DateTimeFormat('default', { month: 'long' });
	const MONTHS: Date[] = [...Array(12).keys()].map((n) => {
		return new Date(new Date().getFullYear(), n, 0);
	});
	let selectedMonth = new Date(new Date()).getMonth();
	let calendarMatrix = [
		[
			{ value: 31, isInMonth: false },
			{ value: 1, isInMonth: true },
			{ value: 2, isInMonth: true },
			{ value: 3, isInMonth: true },
			{ value: 4, isInMonth: true },
			{ value: 5, isInMonth: true },
			{ value: 6, isInMonth: true }
		],
		[
			{ value: 7, isInMonth: true },
			{ value: 8, isInMonth: true },
			{ value: 9, isInMonth: true },
			{ value: 10, isInMonth: true },
			{ value: 11, isInMonth: true },
			{ value: 12, isInMonth: true },
			{ value: 13, isInMonth: true }
		]
	];
	// TODO: List of pairs of ints instead of dumb matrix
	// Also todo: multi-month support
	let selected: boolean[][] = [...Array(calendarMatrix.length).keys()].map(() => {
		return [...Array(calendarMatrix[0].length).keys()].map(() => false);
	});
	// let selectedDay: number = 1;
	// $: selected = `${selectedMonth} ${selectedDay}`;
	export let selectedDates: string[] = [];
	// $: selectedDates = convertSelectedDate()
	// function select() {
	// 	selectedDates = [selected, ...selectedDates];
	// 	selectedMonth = null;
	// 	selectedDay = 1;
	// }
</script>

<div class="form-control">
	<div class="dropdown">
		<label class="input-group">
			<input
				type="text"
				placeholder="Select a date"
				class="input input-bordered w-full max-w-xs"
				readonly
			/><span
				><svg
					xmlns="http://www.w3.org/2000/svg"
					class="inline h-5 w-5"
					viewBox="0 0 20 20"
					fill="currentColor"
				>
					<path
						fill-rule="evenodd"
						d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z"
						clip-rule="evenodd"
					/>
				</svg></span
			></label
		>
		<div tabindex="0" class="dropdown-content card card-compact w-fit shadow bg-base-100">
			<div class="card-body">
				<h3 class="card-title mx-auto">
					<button
						class="btn btn-ghost mr-auto"
						disabled={selectedMonth == 0}
						on:click={() => {
							selectedMonth--;
						}}
						><svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-5 w-5"
							viewBox="0 0 20 20"
							fill="currentColor"
						>
							<path
								fill-rule="evenodd"
								d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z"
								clip-rule="evenodd"
							/>
						</svg></button
					>
					<select class="select select-bordered" bind:value={selectedMonth}>
						{#each MONTHS as month}
							<!-- We could also make the value the index if performance matters -->
							<option value={month.getMonth()}>{monthFormatter.format(month)}</option>
						{/each}
					</select>
					<button
						class="btn btn-ghost left-auto right-0"
						disabled={selectedMonth == 11}
						on:click={() => {
							selectedMonth++;
						}}
						><svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-5 w-5"
							viewBox="0 0 20 20"
							fill="currentColor"
						>
							<path
								fill-rule="evenodd"
								d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
								clip-rule="evenodd"
							/>
						</svg></button
					>
				</h3>
				<table class="table w-full">
					<!-- head -->
					<thead>
						<tr class="[&>*]:normal-case">
							<th>Sun</th>
							<th>Mon</th>
							<th>Tue</th>
							<th>Wed</th>
							<th>Thu</th>
							<th>Fri</th>
							<th>Sat</th>
						</tr>
					</thead>
					<tbody>
						{#each calendarMatrix as week, i}
							<tr>
								{#each week as day, j}
									<td
										class="table-cell btn btn-ghost text-center border-1 border-base-200"
										class:btn-disabled={!day.isInMonth}
										class:btn-active={selected[i][j]}
										on:click={() => {
											selected[i][j] = !selected[i][j];
										}}>{day.value}</td
									>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>
