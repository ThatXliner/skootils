<script lang="ts">
	import DateInput from '$lib/DateInput.svelte';
	import HomeButton from '$lib/HomeButton.svelte';

	let dateType: 'latest' | 'custom' = 'latest';
	let dateList: string[] = [];
	let quarter: string;
	$: if (quarter == 'Latest') {
		window.sessionStorage.removeItem('quarter');
	} else {
		window.sessionStorage.setItem('quarter', quarter);
	}
	$: if (dateType === 'custom') {
		window.sessionStorage.setItem('config', JSON.stringify(dateList));
	} else {
		window.sessionStorage.removeItem('config');
	}
</script>

<main class="ml-3 mt-2">
	<HomeButton />
	<div class="mx-auto w-3/4">
		<h1 class="text-center text-4xl font-bold">Scrape lesson plans</h1>
		<p class="leading-relaxed text-xl">
			This tool was literally build to scrape learn@vcs lesson plans for their assignments. Now,
			choose the <b> date(s) of the lesson plan(s)</b> you want to download and extract dates from.
		</p>
	</div>
	<div class="w-full flex">
		<div class="w-1/2">
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
		</div>
		<div class="mx-auto">
			<div class="form-control">
				<label class="label" for="quarter">
					<span class="label-text">Lesson plan quarter</span>
				</label>
				<select class="select select-lg select-bordered" bind:value={quarter}>
					<option selected>Latest</option>
					<option>1</option>
					<option>2</option>
					<option>3</option>
					<option>4</option>
				</select>
				<label for="quarter" class="label"
					><span class="label-text-alt">Which quarter is it in?</span></label
				>
			</div>
		</div>
	</div>

	{#if dateType == 'custom'}
		<DateInput bind:selectedDates={dateList} />
	{/if}

	<a
		class="my-5 btn btn-primary"
		href="/learnatvcs/scrape"
		class:btn-disabled={dateType == 'custom' && dateList.length == 0}>Alright, let's go</a
	>
</main>
