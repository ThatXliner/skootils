<script lang="ts">
	// import Card from '$lib/Card.svelte';
	import { onMount } from 'svelte';

	import DOMPurify from 'dompurify'; // I did this because you never know

	let output: { [key: string]: { [key: string]: string } } | null = null;
	let dates: string[];
	let selectedDate: string;
	let selectedClassKey: string;
	let DATETYPE: string[] | null;

	onMount(() => {
		output = JSON.parse(window.sessionStorage.getItem('output') ?? 'null');
		if (output === null) return;
		dates = Object.keys(output);
		selectedDate = dates[0];
		selectedClassKey = Object.keys(output[selectedDate])[0];
		DATETYPE = JSON.parse(window.sessionStorage.getItem('config') ?? 'null');
	});
</script>

{#if output === null}
	<p>Oops, something went wrong</p>
{:else}
	<div class="border-b-2">
		<div class="navbar">
			<div class="navbar-start">
				<span class="pl-4 font-semibold text-xl">Results</span>
			</div>
			<div class="navbar-center">
				{#if DATETYPE === null}
					<span class="btn btn-info">Latest</span>
				{:else if dates.length == 1}
					<span class="btn btn-info">{selectedDate}</span>
				{:else}
					<div class="dropdown dropdown-hover">
						<label for="date-picker" tabindex="0" class="btn btn-ghost btn-outline"
							>{selectedDate}</label
						>
						<ul
							id="date-picker"
							tabindex="0"
							class="bg-base-100 dropdown-content menu p-2 shadow rounded-box w-max"
						>
							{#each dates as date}
								<li>
									<span
										on:click={() => {
											selectedDate = date;
											return false;
										}}
										class:active={date == selectedDate}>{date}</span
									>
								</li>
							{/each}
						</ul>
					</div>
				{/if}
			</div>
			<div class="navbar-end">
				<!-- TODO: Use home icon -->
				<a class="btn btn-neutral" href="/"
					><svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						stroke-width="2"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
						/>
					</svg></a
				>
			</div>
		</div>
	</div>
	<div class="flex max-w-screen">
		<!-- Class selection. TODO: make it fixed -->
		<ul class="min-w-fit min-h-screen bg-base-200 menu p-4">
			<!-- <li class="menu-title"><span>View lesson plan</span></li> -->
			{#each Object.keys(output[selectedDate]) as key}
				<!-- TODO: Add nested TOC thing -->
				<li>
					<span
						on:click={() => {
							selectedClassKey = key;
						}}
						class:active={key == selectedClassKey}>{key}</span
					>
				</li>
			{/each}
		</ul>
		<!-- TODO: figure best, most optimal format -->
		<div class="block ml-3 mt-2 w-full scroll-smooth">
			<!-- <div class="my-2 py-4 bg-base-200 flex justify-evenly flex-wrap rounded-box">
				<Card
					title="Lesson plans"
					summary="View the lesson plans with important information intelligently highlighted"
					action="View"
					href="#plans"
				/>
				<Card title="Summarize" summary="Generate a summary using AI" href="#summarize" />
				<Card
					title="Important dates"
					summary="Search the lesson plan for due dates and assignments and automatically add those to
							your calandar"
					action="Export"
					href="#dates"
				/>
			</div> -->
			<div id="plans">
				<h3 class="text-2xl font-bold">Lesson plans</h3>
				<p>Also highlights dates (beta)</p>
				<!-- See the `mr-3`? Maybe move that to the parent div? -->
				<article
					class="mr-3 my-3 pl-2 overflow-y-scroll max-w-none max-h-96 prose border-4 rounded-box prose-headings:my-3"
				>
					{@html DOMPurify.sanitize(output[selectedDate][selectedClassKey], {
						ADD_TAGS: ['iframe'],
						ADD_ATTR: ['target']
					})}
				</article>
			</div>
			<div id="summarize">
				<h3 class="text-2xl font-bold">AI Summarizer</h3>
				<p>TODO</p>
			</div>
			<div id="dates">
				<h3 class="text-2xl font-bold">Export dates</h3>
				<p>TODO</p>
			</div>
		</div>
	</div>
{/if}

<style lang="postcss">
	:global([data-highlight='true']) {
		@apply btn btn-warning btn-sm;
	}
</style>
