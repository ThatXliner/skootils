<script lang="ts">
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import HomeButton from '$lib/HomeButton.svelte';
	import Chart from 'chart.js/auto';
	const CHARTOPTIONS = {
		// Maybe I'll do borderJoinStyle
		maintainAspectRatio: false,
		pointRadius: 7,
		borderWidth: 5, // Connection line's width relies on border
		pointBorderWidth: 0
	};

	let quarters: string[];
	let currentQuarter: string;
	type ClassInfo = {
		// Class name
		name: string;
		class_name: string;
		email: string;
		quarter_info: {
			name: string;
			overall_grade: { name: string; percent: string };
			scores: {
				due_dates: string;
				grade: string;
				name: string;
				percent: Number;
				score: string;
				type: string;
				// type: 'Classwork' | 'Homework' | 'Test' | 'Quiz';
			}[];
		};
	};
	let selectedClass: ClassInfo;
	let data: {
		// Quarter (could be "Latest (X)")
		[key: string]: {
			// Period: class
			[key: string]: ClassInfo;
		};
	} | null = null;
	let allTimeGradeData: object; // multi quarter data
	let quarterGradeData: object; // quarter data for class
	$: if (currentQuarter !== undefined) {
		let command = new Command('powerschool-history-alltime', [
			'-um',
			'powerschool.history.alltime',
			currentQuarter.startsWith('Latest') ? currentQuarter.match(/\((\w+)\)/)![1] : currentQuarter
		]);
		command.on('error', (error) => console.error(`command error: "${error}"`));
		command.stderr.on('data', (line) => console.log(`command stderr: "${line}"`));
		command.stdout.on('data', (line) => {
			allTimeGradeData = JSON.parse(line);
		});
		command.spawn();
	}
	$: if (selectedClass !== undefined) {
		let command = new Command('powerschool-history-class', [
			'-um',
			'powerschool.history.alltime',
			currentQuarter.startsWith('Latest') ? currentQuarter.match(/\((\w+)\)/)![1] : currentQuarter,
			selectedClass['class_name']
		]);
		command.on('error', (error) => console.error(`command error: "${error}"`));
		command.stderr.on('data', (line) => console.log(`command stderr: "${line}"`));
		command.stdout.on('data', (line) => {
			quarterGradeData = JSON.parse(line);
		});
		command.spawn();
	}
	let _allTimeChartElement: HTMLCanvasElement;
	let allTimeChart: Chart;
	$: if (allTimeGradeData !== undefined && _allTimeChartElement !== undefined) {
		const ctx = _allTimeChartElement.getContext('2d');
		if (ctx === null) console.error('No canvas wth');
		if (allTimeChart !== undefined) {
			allTimeChart.destroy();
		}
		// @ts-ignore
		allTimeChart = new Chart(ctx, {
			type: 'line',
			data: allTimeGradeData,
			options: CHARTOPTIONS
		});
	}

	let _quarterChartElement: HTMLCanvasElement;
	let quarterChart: Chart;
	$: if (quarterGradeData !== undefined && _quarterChartElement !== undefined) {
		const ctx = _quarterChartElement.getContext('2d');
		if (ctx === null) console.error('No canvas wth');
		if (quarterChart !== undefined) {
			quarterChart.destroy();
		}
		// @ts-ignore
		quarterChart = new Chart(ctx, {
			type: 'line',
			data: quarterGradeData,
			options: CHARTOPTIONS
		});
	}
	function calculateGPA(data: { [key: string]: ClassInfo }): number {
		let scores: number[] = [];
		const GRADE_NAME_TO_GPA = {
			'A+': 4,
			A: 4,
			'A-': 3.7,
			'B+': 3.3,
			B: 3,
			'B-': 2.7,
			'C+': 2.3,
			C: 2.0,
			'C-': 1.7,
			'D+': 1.3,
			D: 1.0
		};
		for (let info of Object.values(data)) {
			let gradeName = info['quarter_info']['overall_grade']['name'];
			if (gradeName !== 'N/A') {
				// @ts-ignore
				scores.push(GRADE_NAME_TO_GPA[info['quarter_info']['overall_grade']['name']] ?? 0);
			}
		}
		return scores.reduce((a, b) => a + b, 0) / scores.length;
	}
	$: gpa =
		currentQuarter !== undefined && data !== null ? calculateGPA(data[currentQuarter]) : null;

	onMount(() => {
		data = JSON.parse(window.sessionStorage.getItem('output') ?? 'null');
		if (data === null) return;
		quarters = Object.keys(data);
		currentQuarter = quarters[0];
	});
</script>

{#if data === null}
	<p>Uh oh, an error has occured</p>
{:else}
	<!-- Modal stuff -->
	<input type="checkbox" id="my-modal" class="modal-toggle" />

	<div class="modal">
		<div class="modal-box">
			<h3 class="font-bold text-lg">{selectedClass?.class_name}</h3>
			<h4 class="font-bold text-md text-center">Grade history this quarter</h4>
			<div class="h-full w-full">
				<canvas bind:this={_quarterChartElement} width="400" height="300" />
			</div>
			<div class="modal-action">
				<label for="my-modal" class="btn">Done</label>
			</div>
		</div>
	</div>
	<!-- Actual content -->
	<div class="navbar border-b-2">
		<div class="navbar-start">
			<span class="pl-4 font-semibold text-xl">Results</span>
		</div>
		<div class="navbar-center">
			{#if quarters.length == 1}
				<span class="btn btn-info">{currentQuarter}</span>
			{:else}
				<div class="dropdown dropdown-hover">
					<label for="date-picker" tabindex="0" class="btn btn-ghost btn-outline"
						>{currentQuarter}</label
					>
					<ul
						id="date-picker"
						tabindex="0"
						class="bg-base-100 dropdown-content menu p-2 shadow rounded-box w-max"
					>
						{#each quarters as quarter}
							<li>
								<span
									on:click={() => {
										currentQuarter = quarter;
										return false;
									}}
									class:active={quarter == currentQuarter}>{quarter}</span
								>
							</li>
						{/each}
					</ul>
				</div>
			{/if}
		</div>
		<div class="navbar-end">
			<HomeButton />
		</div>
	</div>

	<div class="py-2 bg-base-200">
		<!-- <h1 class="text-3xl text-center">Quick look</h1> -->
		<!-- MARK: Grades -->
		<p class="ml-3 badge badge-lg badge-primary">GPA: {gpa}</p>
		<div class="flex flex-wrap justify-evenly">
			{#each Object.values(data[currentQuarter]) as classInfo}
				{@const grade = classInfo['quarter_info']['overall_grade']}
				{#if grade['name'] != 'N/A'}
					{@const gradeNum = +grade['percent']}
					<div
						class="stats m-2 w-fit shadow-xl"
						class:bg-green-400={gradeNum >= 98}
						class:bg-green-300={98 > gradeNum && gradeNum >= 90}
						class:bg-yellow-400={90 > gradeNum && gradeNum >= 70}
						class:bg-red-400={70 > gradeNum}
					>
						<div class="stat">
							<div class="stat-title">
								{classInfo['class_name']}
							</div>
							<div class="stat-value text-center">
								<!-- XXX: we might not even have a swap... if it intrudes user experience -->
								<label class="swap">
									<input type="checkbox" />
									<div class="swap-on">
										{grade['percent']}% <span class="text-sm text-gray-500">{grade['name']}</span>
									</div>
									<div class="swap-off">
										{grade['name']} <span class="text-sm text-gray-500">{grade['percent']}%</span>
									</div>
								</label>
							</div>
							<div class="stat-desc">
								{classInfo['name']}
								<label
									for="my-modal"
									class="ml-3 float-right btn-xs btn btn-primary"
									on:click={() => {
										selectedClass = classInfo;
									}}>More</label
								>
								<!-- <button
									class="ml-3 float-right btn-xs btn btn-primary"
									on:click={() => {
										window.alert('hi');
									}}>More</button
								> -->
							</div>
						</div>
					</div>
				{/if}
			{/each}
		</div>
	</div>
	<h3 class="text-[3.5vw] m-3">All-time grade history</h3>
	<div class="relative p-3 h-xl w-full">
		<canvas bind:this={_allTimeChartElement} width="400" height="300" />
	</div>
{/if}
