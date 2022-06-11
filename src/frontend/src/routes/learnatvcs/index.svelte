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
	<!-- XXX: should we center all of this? -->
	<div class="mt-2 mx-auto w-3/4">
		<h1 class="text-center text-4xl font-bold my-3">Scrape lesson plans</h1>
		<p class="leading-relaxed text-xl">
			This tool was literally build to scrape learn@vcs lesson plans for their assignments. Now,
			choose the <b> date(s) of the lesson plan(s)</b> you want to download and extract dates from.
		</p>
	</div>
	<div class="form-control max-w-sm my-3">
		<label class="label cursor-pointer"
			><span class="text-2xl">Latest dates</span>
			<input
				type="radio"
				class="radio radio-lg radio-primary"
				value="latest"
				bind:group={dateType}
				checked
			/>
		</label>
		<label class="label cursor-pointer"
			><span class="text-2xl">Custom dates</span>
			<input
				type="radio"
				class="radio radio-lg radio-primary"
				value="custom"
				bind:group={dateType}
			/>
		</label>
	</div>

	{#if dateType == 'custom'}
		<DateInput bind:selectedDates={dateList} />
	{/if}
	<!-- We're gonna leave some space for images -->
	<a
		class="my-5 btn btn-primary"
		href="/learnatvcs/scrape"
		class:btn-disabled={dateType == 'custom' && dateList.length == 0}>Alright, let's go</a
	>
</main>
