<script lang="ts">
	import DateInput from '$lib/DateInput.svelte';

	let dateType: 'latest' | 'custom' = 'latest';
	let dateList: string[] = [];
	$: if (dateType === 'custom') {
		window.sessionStorage.setItem('config', JSON.stringify(dateList));
	} else {
		window.sessionStorage.removeItem('config');
	}
</script>

<main class="ml-3">
	<h1 class="text-5xl mt-2">Config</h1>
	<p class="my-1">Desc</p>
	<div class="form-control w-96 my-3">
		<label class="label cursor-pointer"
			><span class="text-lg">Latest dates</span>
			<input
				type="radio"
				class="radio radio-lg radio-primary"
				value="latest"
				bind:group={dateType}
				checked
			/>
		</label>
		<label class="label cursor-pointer"
			><span class="text-lg">Custom dates</span>
			<input
				type="radio"
				class="radio radio-lg radio-primary"
				value="custom"
				bind:group={dateType}
			/>
		</label>
	</div>

	{#if dateType == 'custom'}
		{#if dateList.length > 0}
			<ul id="date-list" class="flex flex-col p-2 border-2 rounded-box w-fit">
				{#each dateList as date, i}
					<!-- TODO: argh where do we put the trash can icon -->
					<li
						class="my-1 btn btn-accent"
						on:click={() => {
							dateList = dateList.slice(0, i).concat(dateList.slice(i + 1));
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
						<span class="mx-1">{date}</span>
					</li>
				{/each}
			</ul>
		{:else}
			<p class="text-red-500 font-bold border-2 dark:bg-neutral p-2 py-5 rounded-box w-fit">
				No dates selected
			</p>
		{/if}

		<DateInput
			on:select={({ detail: date }) => {
				if (dateList.includes(date)) {
					return;
				}
				dateList = [...dateList, date];
			}}
		/>
	{/if}
	<a
		class="my-5 btn btn-primary"
		href="/scrape"
		class:btn-disabled={dateType == 'custom' && dateList.length == 0}>Alright, let's go</a
	>
</main>
