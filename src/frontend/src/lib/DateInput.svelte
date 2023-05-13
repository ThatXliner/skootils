<script lang="ts">
	import AlertWarning from './AlertWarning.svelte';

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
	// Also TODO: multi-month support
	let selected: boolean[][][] = [...Array(12).keys()].map((month) =>
		[...Array(calendarMatrices[month].length).keys()].map(() => {
			return [...Array(calendarMatrices[month][0].length).keys()].map(() => false);
		})
	);
	export let selectedDates: [number, number][] = [];
	// a forest is a group of trees
	let dateForest: { [key: string]: { day: number; raw: [number, number, number] }[] } = {};
	$: {
		let sel = [];
		dateForest = {};
		for (let [month, calendarMatrix] of calendarMatrices.entries()) {
			const monthName = monthFormatter.format(MONTHS[month]);

			for (let [i, row] of calendarMatrix.entries()) {
				for (let [j, column] of row.entries()) {
					if (selected[month][i][j]) {
						if (!(monthName in dateForest)) {
							dateForest[monthName] = [];
						}
						dateForest[monthName] = [
							...dateForest[monthName],
							{ day: column.value, raw: [month, i, j] }
						]; // [month, day]
						sel.push([month, column.value]);
					}
				}
			}
		}
		selectedDates = sel;
	}
</script>

<div class="overflow-x-hidden">
	<div class="flex w-full justify-evenly">
		<div class="card card-compact w-fit">
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
										class="table-cell btn btn-xs btn-ghost text-center border-1 border-base-200"
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
		<div class="divider divider-horizontal" />
		<div class="w-56">
			{#if Object.entries(dateForest).length == 0}
				<AlertWarning message="No dates selected" />
			{:else}
				<ul class="menu h-fit max-h-64 overflow-y-auto p-2 shadow-lg rounded-box">
					{#each Object.entries(dateForest) as [month, days]}
						<!-- {#if days.length > 0} -->
						<li class="menu-title">
							<span>{month}</span>
						</li>
						{#each days as day}
							<li
								on:click={() => {
									selected[day.raw[0]][day.raw[1]][day.raw[2]] = false;
								}}
							>
								<span
									><svg
										xmlns="http://www.w3.org/2000/svg"
										viewBox="0 0 20 20"
										fill="currentColor"
										class="w-5 h-5"
									>
										<path
											fill-rule="evenodd"
											d="M8.75 1A2.75 2.75 0 006 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 10.23 1.482l.149-.022.841 10.518A2.75 2.75 0 007.596 19h4.807a2.75 2.75 0 002.742-2.53l.841-10.52.149.023a.75.75 0 00.23-1.482A41.03 41.03 0 0014 4.193V3.75A2.75 2.75 0 0011.25 1h-2.5zM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4zM8.58 7.72a.75.75 0 00-1.5.06l.3 7.5a.75.75 0 101.5-.06l-.3-7.5zm4.34.06a.75.75 0 10-1.5-.06l-.3 7.5a.75.75 0 101.5.06l.3-7.5z"
											clip-rule="evenodd"
										/>
									</svg>
									{month}
									{day.day}</span
								>
							</li>
						{/each}
					{/each}
				</ul>{/if}
		</div>
	</div>
</div>
