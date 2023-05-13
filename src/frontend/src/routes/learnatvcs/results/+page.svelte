<script lang="ts">
	import HomeButton from '$lib/HomeButton.svelte';
	import { onMount } from 'svelte';

	import DOMPurify from 'dompurify'; // I did this because you never know

	let output: { [key: string]: { [key: string]: { [key: string]: string } } } | null = null;
	let classes: string[];
	let selectedClass: string;
	let selectedQuarter: string;
	let selectedDate: string;
	let mode: 'normal' | 'overview' = 'normal'; // @hmr:keep

	onMount(() => {
		console.log(window.sessionStorage.getItem('output'));
		output = JSON.parse(window.sessionStorage.getItem('output') ?? 'null');
		if (output === null) return;
		classes = Object.keys(output);
		selectedClass = classes[0];
		selectedQuarter = Object.keys(output[selectedClass])[0];
		selectedDate = Object.keys(output[selectedClass][selectedQuarter])[0];
	});

	function getContents(html: string) {
		function recursiveNextSibling(element: Element): string {
			let output = '';
			let cursor: Element | null = element;
			while (cursor && !(cursor.nodeName === 'P' && cursor.querySelector('strong') !== null)) {
				output += cursor.outerHTML;
				cursor = cursor.nextElementSibling;
			}
			return output;
		}
		const parser = new DOMParser();
		let doc = parser.parseFromString(html, 'text/html');
		return Array.from(doc.body.querySelectorAll('h4,h3'))
			.filter((x) => /(turned in)|(deliverables)|(homework)/i.test(x.textContent ?? ''))
			.map(recursiveNextSibling)
			.join('\n');
	}
</script>

{#if output === null}
	<p>Oops, something went wrong</p>
{:else}
	<div class="navbar border-b-2">
		<div class="navbar-start">
			<span class="pl-4 font-semibold text-xl">Results</span>
		</div>
		<div class="navbar-center">
			<div class="space-x-4">
				<div class="dropdown dropdown-hover">
					{#if Object.keys(output[selectedClass]).length == 1}
						<span class="btn btn-accent">{selectedQuarter}</span>
					{:else}
						<label for="quarter-picker" tabindex="0" class="btn btn-ghost btn-outline"
							>{selectedQuarter}</label
						>
						<ul
							id="quarter-picker"
							tabindex="0"
							class="bg-base-100 dropdown-content menu p-2 shadow rounded-box w-max"
						>
							{#each Object.keys(output[selectedClass]) as quarter}
								<li>
									<span
										on:click={() => {
											selectedQuarter = quarter;
											// @ts-ignore
											selectedDate = Object.keys(output[selectedQuarter])[0];
											return false;
										}}
										class:active={quarter == selectedQuarter}>{quarter}</span
									>
								</li>
							{/each}
						</ul>{/if}
				</div>
				<div class="dropdown dropdown-hover">
					{#if Object.keys(output[selectedClass][selectedQuarter]).length == 1}<span
							class="btn btn-accent">{selectedDate}</span
						>
					{:else}
						<label for="date-picker" tabindex="0" class="btn btn-ghost btn-outline"
							>{selectedDate}</label
						>
						<ul
							id="date-picker"
							tabindex="0"
							class="bg-base-100 dropdown-content menu p-2 shadow rounded-box w-max"
						>
							{#each Object.keys(output[selectedClass][selectedQuarter]) as date}
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
						</ul>{/if}
				</div>
			</div>
		</div>
		<div class="navbar-end">
			<HomeButton />
		</div>
	</div>

	<div class="flex max-w-screen">
		{#if mode === 'normal'}
			<ul class="min-w-fit min-h-screen bg-base-200 menu p-4">
				<!-- <li class="menu-title"><span>View lesson plan</span></li> -->
				{#each Object.keys(output) as key}
					{@const className = key.split(/\s+-\s+/)[0]}
					{@const teacher = key.split(/\s+-\s+/)[1]}
					<!-- TODO: Add nested TOC thing -->
					<li>
						<span
							on:click={() => {
								selectedClass = key;
							}}
							class:active={key == selectedClass}
							>{className}
							<span class="text-xs pt-2 text-gray-{key == selectedClass ? '300' : '500'}"
								>{teacher}</span
							></span
						>
					</li>
				{/each}
			</ul>
			<!-- TODO: figure best, most optimal format -->
			<div class="block ml-3 mt-2 w-full">
				<!-- <div class="my-2 py-4 bg-base-200 flex justify-evenly flex-wrap rounded-box">
				<Card
					title="Lesson plans"
					summary="View the lesson plans with important information intelligently highlighted"
					action="View"
					href="#plans"
				/>
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
					<article class="lesson-plan max-h-96 overflow-y-auto">
						{@html DOMPurify.sanitize(output[selectedClass][selectedQuarter][selectedDate], {
							ADD_TAGS: ['iframe'],
							ADD_ATTR: ['target']
						})}
					</article>
				</div>
				<div id="dates">
					<h3 class="text-2xl font-bold">Export dates</h3>
					<p>TODO</p>
				</div>
			</div>
		{:else if mode === 'overview'}
			<div class="w-screen">
				<h3 class="text-3xl text-center py-3">Quick Overview</h3>
				<p class="text-center">
					Show all detected assignments in one page because I haven't got an AI to do it yet.
				</p>

				{#each classes as klass}
					{@const className = klass.split(/\s+-\s+/)[0]}
					{@const teacher = klass.split(/\s+-\s+/)[1]}
					{@const contents = getContents(output[klass][selectedQuarter][selectedDate])}
					<div class="ml-3 p-3">
						<span class="text-xl"
							>{className} <span class="text-xs pt-2 text-gray-500">{teacher}</span></span
						>
						<article class="lesson-plan mt-2">
							{@html DOMPurify.sanitize(contents, {
								ADD_ATTR: ['target']
							})}
						</article>
					</div>
				{/each}
			</div>
		{/if}
	</div>
	{#if mode === 'normal'}
		<button
			class="btn btn-primary fixed my-2 bottom-3 left-3"
			on:click={() => {
				mode = 'overview';
			}}>Quick overview</button
		>
	{:else if mode === 'overview'}
		<button
			class="btn btn-primary sticky my-2 bottom-3 left-3"
			on:click={() => {
				mode = 'normal';
			}}>Back to normal view</button
		>
	{/if}
{/if}

<style lang="postcss">
	#plans :global([data-highlight='true']) {
		@apply btn btn-warning btn-sm;
	}
	#plans :global(h2) {
		margin-top: 1rem;
		margin-bottom: 1rem;
	}
	#plans :global(h3) {
		margin-top: 0.1rem;
		margin-bottom: 0.1rem;
	}
	#plans :global(h4) {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}
	#plans :global(a::after) {
		content: url("data:image/svg+xml,%3Csvg width='1.25em' height='1.25em' fill='currentColor' viewBox='0 0 20 20' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z'/%3E%3Cpath d='M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z'/%3E%3C/svg%3E%0A");
	}
	.lesson-plan {
		@apply prose rounded-box border-4 p-2 pl-3;
	}
</style>
