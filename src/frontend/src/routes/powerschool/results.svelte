<script lang="ts">
	import { onMount } from 'svelte';
	import HomeButton from '$lib/HomeButton.svelte';
	let quarters: string[];
	let currentQuarter: string;
	let data: {
		[key: string]: {
			[key: string]: {
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
		};
	} | null = null;
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
	<div class="border-b-2">
		<div class="navbar">
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
	</div>
	<div class="py-2 bg-base-200">
		<div class="flex flex-wrap justify-evenly">
			{#each Object.entries(data[currentQuarter]) as className}
				{@const classInfo = className[1]}
				{@const grade = classInfo['quarter_info']['overall_grade']}
				{#if grade['name'] != 'N/A'}
					{@const gradeNum = +grade['percent']}
					<div
						class="stats shadow m-2 w-48"
						class:bg-green-400={gradeNum >= 98}
						class:bg-green-300={98 > gradeNum && gradeNum >= 90}
						class:bg-yellow-400={90 > gradeNum && gradeNum >= 70}
						class:bg-red-400={70 > gradeNum}
					>
						<div class="stat">
							<div class="stat-title">{classInfo['class_name']}</div>
							<div class="stat-value">
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
							<div class="stat-desc">{classInfo['name']}</div>
						</div>
					</div>
				{/if}
			{/each}
		</div>
	</div>
{/if}
