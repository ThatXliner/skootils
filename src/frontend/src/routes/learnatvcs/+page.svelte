<script lang="ts">
	/// Status: all done, a bit more surrounding styling
	/// could be done. Need to cache (although that could be done in backend)
	/// and keep command in store? prob not
	import AccountInput from '$lib/AccountInput.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import DateInput from '$lib/DateInput.svelte';
	import HomeButton from '$lib/HomeButton.svelte';
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';

	let dateType: 'latest' | 'custom' = 'latest';
	let dateList: string[] = [];
	let quarter: string;
	type Credentials = { username: string; password: string } | null;
	let credentials: Writable<Credentials> = writable(null);
	let getCredentials: Promise<Credentials>;
	onMount(() => {
		getCredentials = invoke('get_credentials').then((result) => {
			$credentials = JSON.parse(result || 'null');
		});
	});
	let scraper = null;
</script>

{#await getCredentials then}
	{#if $credentials == null}
		<div class="modal modal-open">
			<AccountInput
				on:input={(event) => {
					invoke('set_credentials', event.detail)
						.then(() => {
							$credentials = event.detail;
						})
						.catch(console.log);
				}}
			/>
		</div>
	{/if}
	<main class="ml-3 mt-2">
		<HomeButton />
		<div class="mx-auto w-3/4">
			<h1 class="text-center text-4xl font-bold">Scrape lesson plans</h1>
			<p class="leading-relaxed text-xl">
				This tool was literally built to scrape learn@vcs lesson plans for their assignments. Now,
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

		<btn
			class="my-5 btn btn-primary"
			on:click={() => {
				scraper = invoke('scrape_plans', {
					...$credentials,
					quarter: { type: quarter === 'Latest' ? 'Latest' : parseInt(quarter) },
					dates: { type: dateType == 'latest' ? 'Latest' : dateList }
				}).then((output) => {
					window.sessionStorage.setItem('output', JSON.stringify(output));
					window.location.assign('/learnatvcs/results');
				});
			}}
			class:btn-disabled={scraper != undefined ||
				$credentials == null ||
				(dateType == 'custom' && dateList.length == 0)}>Alright, let's go</btn
		>
		{#if scraper != undefined}
			<p>Scraper is running!!</p>
		{/if}
	</main>
{/await}
