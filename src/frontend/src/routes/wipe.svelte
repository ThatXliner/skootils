<script lang="ts">
	import HomeButton from '$lib/HomeButton.svelte';
	import { getNotificationsContext } from 'svelte-notifications';

	const { addNotification } = getNotificationsContext();

	const CHOICES = {
		'Long term data': {
			description: 'Should be the same every school year',
			choices: ['Your name', 'Your credentials']
		},
		'Yearly trash': {
			description: 'Every year you get new classes...',
			choices: ['All PowerSchool history', 'Teacher information']
		}
	};
	let chosen: string[] = [];
	$: console.log(chosen);
</script>

<div class="hero min-h-screen bg-base-200">
	<HomeButton extraClasses="mr-auto mb-auto ml-3 mt-3" />
	<div class="hero-content flex-col">
		<div class="text-center">
			<h1 class="text-5xl font-bold">Wipe Application Data (forever!)</h1>
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
				<div class="form-control mt-6">
					<button
						class="btn btn-error"
						on:click={() => {
							addNotification({
								text: 'heheheha',
								position: 'top-right',
								type: 'success',
								removeAfter: 4000
							});
						}}>Wipe!</button
					>
				</div>
			</div>
		</div>
	</div>
</div>
