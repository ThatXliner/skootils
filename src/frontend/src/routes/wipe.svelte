<script lang="ts">
	import HomeButton from '$lib/HomeButton.svelte';
	import { fade } from 'svelte/transition';

	const CHOICES = {
		'Long term data': {
			description: 'Should be the same every school year',
			choices: ['Your name and grade', 'Your credentials']
		},
		'Yearly trash': {
			description: 'Every year you get new classes...',
			choices: ['All PowerSchool history', 'Teacher information']
		}
	};
	let chosen: string[] = [];
	let notifId: number | null = null;
	function clearNotif(cancelled: boolean) {
		if (notifId === null) return;
		clearTimeout(notifId);
		notifId = null;
		if (!cancelled) {
			window.alert('TODO');
		}
	}
</script>

<!-- Modal stuff -->
<input type="checkbox" id="my-modal" class="modal-toggle" />
<div class="modal">
	<div class="modal-box">
		<h3 class="font-bold text-lg">Confirm</h3>
		<p class="py-4">
			Are you <b>sure</b> you want to wipe X items permanently? It will be gone forever! (You will reclaim
			X megabytes of storage)
		</p>
		<div class="modal-action">
			<label for="my-modal" class="btn btn-neutral">Cancel</label>
			<label
				for="my-modal"
				class="btn btn-error"
				on:click={() => {
					notifId = window.setTimeout(() => {
						clearNotif(false);
					}, 4000);
				}}>Yes, delete it now</label
			>
		</div>
	</div>
</div>
<!-- Actual body -->
<div class="hero min-h-screen bg-base-200">
	<!-- TODO: multiple notifs -->
	{#if notifId !== null}
		<div class="alert alert-success shadow-lg top-2 right-2 max-w-fit fixed z-40" out:fade>
			<div>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="stroke-current flex-shrink-0 h-6 w-6"
					fill="none"
					viewBox="0 0 24 24"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
					/></svg
				>
				<span>Wiped X items</span> |
				<button
					class="link link-primary"
					on:click={() => {
						clearNotif(true);
					}}>Undo</button
				>
				<button
					on:click={() => {
						clearNotif(false);
					}}
					><svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						viewBox="0 0 20 20"
						fill="currentColor"
					>
						<path
							fill-rule="evenodd"
							d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
							clip-rule="evenodd"
						/>
					</svg></button
				>
			</div>
		</div>
	{/if}
	<HomeButton extraClasses="mr-auto mb-auto ml-3 mt-3" />
	<div class="hero-content flex-col">
		<div class="text-center">
			<h1 class="text-5xl font-bold">Wipe Data (forever!)</h1>
			<p class="py-6">
				Hello brave user, it's been heard that you would like to wipe the application data files,
				whether that be the caches, configuration, or whatever you would like. Maybe you wanted to
				uninstall this app cleanly or maybe you are a storage-minimizing fanatic. Who knows?
			</p>
			<p class="text-lg">Here's your deletion choices. Pick your poison.</p>
		</div>
		<div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
			<div class="card-body">
				<div class="form-control">
					{#each Object.entries(CHOICES) as entry}
						<span class="text-stone-600 font-bold text-sm"
							>{entry[0]}
							<div class="tooltip" data-tip={entry[1]['description']}>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
									class="w-4 h-4 stroke-current text-info"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
									/></svg
								>
							</div></span
						>
						{#each entry[1]['choices'] as choice}
							<label class="label cursor-pointer">
								<span class="label-text text-lg">{choice}</span>
								<input
									type="checkbox"
									bind:group={chosen}
									value={choice}
									class="checkbox checkbox-lg checkbox-primary"
								/>
							</label>
						{/each}
					{/each}
				</div>
				<!-- TODO: multiple notifs -->
				<div class="form-control mt-6">
					<label
						for="my-modal"
						class="btn btn-error modal-button"
						class:btn-disabled={notifId !== null || chosen.length == 0}>Wipe!</label
					>
				</div>
			</div>
		</div>
	</div>
</div>
