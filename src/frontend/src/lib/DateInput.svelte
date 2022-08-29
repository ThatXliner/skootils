<script lang="ts">
	const YEAR = new Date().getFullYear();
	const monthFormatter = new Intl.DateTimeFormat('default', { month: 'long' });
	const MONTHS: Date[] = [...Array(12).keys()].map((n) => {
		// First days of the month
		return new Date(YEAR, n, 1);
	});
	function getLastDay(date: number): Date {
		let newDate = new Date();
		newDate.setMonth(date + 1);
		// last day of previous month
		newDate.setDate(0);
		return newDate;
	}

	let selectedMonth = new Date().getMonth();
	function subtractOneMonth(month: number) {
		if (month == 1) {
			month = 12;
			// decrement year
		} else {
			month--;
		}
		return month;
	}
	function generateCalendarMatrix(month: number) {
		let calMatrix = [];
		let day = 1;
		function generateFirstWeek() {
			// fill up the first week
			let initialOffset = MONTHS[month].getDay();
			let firstWeek = [];
			const DAYSINLASTMONTH = getLastDay(subtractOneMonth(month)).getDate();
			let days = DAYSINLASTMONTH - initialOffset + 1;
			while (days != DAYSINLASTMONTH + 1) {
				firstWeek.push({ value: days, isInMonth: false });
				days++;
			}
			for (let i = initialOffset; i < 7; i++) {
				firstWeek.push({ value: day++, isInMonth: true });
			}
			return firstWeek;
		}

		calMatrix.push(generateFirstWeek());
		let week = [];
		while (day < getLastDay(month).getDate() + 1) {
			week.push({ value: day, isInMonth: true });
			if (week.length == 7) {
				calMatrix.push(week);
				week = [];
			}
			day++;
		}
		// fill last week
		if (week.length > 0) {
			let day = 1;
			for (let i = week.length; i < 7; i++) {
				week.push({ value: day, isInMonth: false });
				day++;
			}
			calMatrix.push(week);
			week = [];
		}
		return calMatrix;
	}
	let calendarMatrices = [...Array(12).keys()].map((month) => generateCalendarMatrix(month));
	// Also todo: multi-month support
	let selected: boolean[][][] = [...Array(12).keys()].map((month) =>
		[...Array(calendarMatrices[month].length).keys()].map(() => {
			return [...Array(calendarMatrices[month][0].length).keys()].map(() => false);
		})
	);
	export let selectedDates: string[] = [];
	$: {
		let sel = [];
		for (let [month, calendarMatrix] of calendarMatrices.entries()) {
			for (let [i, row] of calendarMatrix.entries()) {
				for (let [j, column] of row.entries()) {
					if (selected[month][i][j]) {
						sel.push(`${monthFormatter.format(MONTHS[month])} ${column.value}`);
					}
				}
			}
		}
		selectedDates = sel;
	}
	$: placeholder = selectedDates.join(', ') || 'Select dates';
</script>

<div class="form-control">
	<div class="dropdown">
		<label class="input-group">
			<input type="text" {placeholder} class="input input-bordered w-full max-w-xs" readonly /><span
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
						{#each calendarMatrices[selectedMonth] as week, i}
							<tr>
								{#each week as day, j}
									<td
										class="table-cell btn btn-ghost text-center border-1 border-base-200"
										class:btn-disabled={!day.isInMonth}
										class:btn-active={selected[selectedMonth][i][j]}
										on:click={() => {
											selected[selectedMonth][i][j] = !selected[selectedMonth][i][j];
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
