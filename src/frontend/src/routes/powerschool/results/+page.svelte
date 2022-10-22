<script lang="ts">
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import { writable, type Writable } from 'svelte/store';
	import HomeButton from '$lib/HomeButton.svelte';
	import WhatIf from '$lib/WhatIf.svelte';
	import * as pkg from 'chart.js';
	const {
		Chart,
		ArcElement,
		LineElement,
		BarElement,
		PointElement,
		BarController,
		BubbleController,
		DoughnutController,
		LineController,
		PieController,
		PolarAreaController,
		RadarController,
		ScatterController,
		CategoryScale,
		LinearScale,
		LogarithmicScale,
		RadialLinearScale,
		TimeScale,
		TimeSeriesScale,
		Decimation,
		Filler,
		Legend,
		Title,
		Tooltip,
		SubTitle
	} = pkg;
	Chart.register(
		ArcElement,
		LineElement,
		BarElement,
		PointElement,
		BarController,
		BubbleController,
		DoughnutController,
		LineController,
		PieController,
		PolarAreaController,
		RadarController,
		ScatterController,
		CategoryScale,
		LinearScale,
		LogarithmicScale,
		RadialLinearScale,
		TimeScale,
		TimeSeriesScale,
		Decimation,
		Filler,
		Legend,
		Title,
		Tooltip,
		SubTitle
	);
	const CHARTOPTIONS = {
		// Maybe I'll do borderJoinStyle
		maintainAspectRatio: false,
		pointRadius: 7,
		borderWidth: 5, // Connection line's width relies on border
		pointBorderWidth: 0
	};

	let quarters: string[];
	let currentQuarter: Writable<string> = writable();
	type ClassInfo = {
		// Class name
		name: string;
		class_name: string;
		email: string;
		quarter_info: {
			name: string;
			overall_grade: { name: string; percent: number };
			scores: {
				due_date: string;
				description?: string;
				comment?: string;
				grade: string;
				name: string;
				flags: number;
				percent?: number;
				score: { total: number; recieved?: number };
				type: string;
				_raw: object;
				// type: 'Classwork' | 'Homework' | 'Test' | 'Quiz';
			}[];
		};
	};
	let selectedClass: Writable<ClassInfo> = writable();
	let data: {
		// Quarter (could be "Latest (X)")
		[key: string]: {
			// Period: class
			[key: string]: ClassInfo;
		};
	} | null = null;
	let allTimeGradeData: object; // multi quarter data
	let quarterGradeData: object; // quarter data for class
	currentQuarter.subscribe((value) => {
		if (value === undefined) return;
		let command = Command.sidecar('../../powerschool/dist/alltime', [
			value.startsWith('Latest') ? value.match(/\((\w+)\)/)![1] : value
		]);
		command.on('error', (error) => console.error(`command error: "${error}"`));
		command.stderr.on('data', (line) => console.log(`command stderr: "${line}"`));
		command.stdout.on('data', (line) => {
			allTimeGradeData = JSON.parse(line);
		});
		command.spawn();
	});
	selectedClass.subscribe((value) => {
		if (value === undefined || $currentQuarter === undefined) return;
		let command = Command.sidecar('../../powerschool/dist/for_class', [
			$currentQuarter.startsWith('Latest')
				? $currentQuarter.match(/\((\w+)\)/)![1]
				: $currentQuarter,
			value['class_name']
		]);
		command.on('error', (error) => console.error(`command error: "${error}"`));
		command.stderr.on('data', (line) => console.log(`command stderr: "${line}"`));
		command.stdout.on('data', (line) => {
			quarterGradeData = JSON.parse(line);
		});
		command.spawn();
	});
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

	onMount(() => {
		data = JSON.parse(window.sessionStorage.getItem('output') ?? 'null');
		if (data === null) return;
		quarters = Object.keys(data);
		$currentQuarter = quarters[0];
	});
</script>

{#if data === null}
	<p>Uh oh, an error has occured</p>
{:else}
	<!-- Modal stuff -->
	<input type="checkbox" id="my-modal" class="modal-toggle" />

	<div class="modal">
		<div class="modal-box">
			<h3 class="font-bold text-lg">{$selectedClass?.class_name}</h3>
			<details>
				<summary>Grade history this quarter</summary>
				<div class="h-full w-full">
					<canvas bind:this={_quarterChartElement} width="400" height="300" />
				</div>
			</details>
			<details>
				<!-- TODO -->
				<summary
					>What if? <div
						class="tooltip tooltip-right"
						data-tip="Calculations may be incorrect because we are currently disregarding possible weighting"
					>
						<span class="badge badge-warning">Beta</span>
					</div></summary
				>
				<WhatIf
					currentScore={$selectedClass?.quarter_info.overall_grade.percent}
					assignmentCount={$selectedClass?.quarter_info.scores.length}
				/>
			</details>

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
				<span class="btn btn-info">{$currentQuarter}</span>
			{:else}
				<div class="dropdown dropdown-hover">
					<label for="date-picker" tabindex="0" class="btn btn-ghost btn-outline"
						>{$currentQuarter}</label
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
										$currentQuarter = quarter;
										return false;
									}}
									class:active={quarter == $currentQuarter}>{quarter}</span
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
		<div class="flex flex-wrap justify-evenly">
			{#each Object.values(data[$currentQuarter]) as classInfo}
				{@const grade = classInfo['quarter_info']['overall_grade']}
				{#if grade['name'] !== null}
					{@const gradeNum = +grade['percent']}
					<div
						class="stats m-2 w-fit shadow-xl dark:text-black"
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
										$selectedClass = classInfo;
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
