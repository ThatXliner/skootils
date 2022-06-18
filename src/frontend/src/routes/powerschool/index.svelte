<!-- @hmr:keep-all -->
<script lang="ts">
	import AccountInput from '$lib/AccountInput.svelte';
	import QuarterInput from '$lib/QuarterInput.svelte';
	import HomeButton from '$lib/HomeButton.svelte';

	import { Command, type Child } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';

	let command;
	let child: Promise<Child>;
	let hasCredentials: boolean | null = null;
	let progress: { [key: string]: [number, number] };
	let quarterChoices: string[];

	onMount(() => {
		window.sessionStorage.removeItem('output');
		command = new Command('powerschool');
		command.on('close', (data) => {
			console.log(`command finished with code ${data.code} and signal ${data.signal}`);
			if (window.sessionStorage.getItem('output') !== null) {
				window.location.assign('/powerschool/results');
			}
		});
		command.on('error', (error) => console.error(`command error: "${error}"`));
		command.stdout.on('data', (line) => {
			let input = JSON.parse(line);
			if (typeof input === 'boolean') {
				hasCredentials = input;
			} else if (Array.isArray(input)) {
				quarterChoices = input;
			} else if (input['type'] === 'progress') {
				progress = input['content'];
			} else {
				window.sessionStorage.setItem('output', line);
			}
		});
		command.stderr.on('data', (line) => console.log(`command stderr: "${line}"`));
		child = command.spawn();
		return () => {
			// XXX: What about onDestroy?
			(async function () {
				await (await child).kill();
			})();
		};
	});
</script>

<div class="hero min-h-screen bg-base-200">
	<HomeButton extraClasses="mr-auto mb-auto ml-3 mt-3" />
	<div class="hero-content flex-row-reverse justify-evenly">
		<div class="ml-4 text-left">
			<h1 class="text-5xl font-bold">PowerSchool++</h1>
			<p class="py-6 max-w-md">
				Ever thought PowerSchool was missing something? Was it the "Show GPA" option that VCS
				purposely disables? Or is it something more strategical, maybe a "can I tank this
				assignment"?
			</p>
		</div>
		{#await child then child}
			{#if hasCredentials === false}
				<AccountInput
					on:input={(event) => {
						child.write(JSON.stringify(event.detail) + '\n');
						hasCredentials = true;
					}}
				/>
			{:else if hasCredentials === true}
				{#if progress === undefined && quarterChoices === undefined}
					<svg
						class="h-1/5 w-1/5 inline fill-warning"
						enable-background="new 0 0 50 50"
						version="1.1"
						viewBox="0 0 50 50"
						xml:space="preserve"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="m43.935 25.145c0-10.318-8.364-18.683-18.683-18.683-10.318 0-18.683 8.365-18.683 18.683h4.068c0-8.071 6.543-14.615 14.615-14.615s14.615 6.543 14.615 14.615h4.068z"
						>
							<animateTransform
								attributeName="transform"
								attributeType="xml"
								dur="0.6s"
								from="0 25 25"
								repeatCount="indefinite"
								to="360 25 25"
								type="rotate"
							/>
						</path>
					</svg>
					<h1 class="text-2xl">Logging in</h1>
				{:else if progress === undefined && quarterChoices !== undefined}
					<div>
						<QuarterInput
							{quarterChoices}
							on:submit={({ detail }) => {
								child.write(JSON.stringify(detail) + '\n');
							}}
						/>
					</div>
				{:else if progress !== undefined}
					<div class="w-full">
						<h1 class="text-2xl">Scraping...</h1>
						<ul class="text-xl m-3">
							{#each Object.entries(progress) as quarter_progress (quarter_progress[0])}
								{@const finished = quarter_progress[1]}
								{@const name = quarter_progress[0]}
								<li>
									{#if finished[1] === -1}
										<svg
											xmlns="http://www.w3.org/2000/svg"
											class="h-12 w-12 inline"
											viewBox="0 0 20 20"
											fill="currentColor"
										>
											<path
												fill-rule="evenodd"
												d="M10 18a8 8 0 100-16 8 8 0 000 16zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z"
												clip-rule="evenodd"
											/>
										</svg>
										{name}
									{:else if finished[0] == finished[1]}
										<svg
											xmlns="http://www.w3.org/2000/svg"
											class="h-12 w-12 inline fill-success"
											viewBox="0 0 20 20"
											fill="currentColor"
										>
											<path
												fill-rule="evenodd"
												d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
												clip-rule="evenodd"
											/>
										</svg>
										{name}
									{:else}
										<svg
											class="h-12 w-12 inline fill-warning"
											enable-background="new 0 0 50 50"
											version="1.1"
											viewBox="0 0 50 50"
											xml:space="preserve"
											xmlns="http://www.w3.org/2000/svg"
										>
											<path
												d="m43.935 25.145c0-10.318-8.364-18.683-18.683-18.683-10.318 0-18.683 8.365-18.683 18.683h4.068c0-8.071 6.543-14.615 14.615-14.615s14.615 6.543 14.615 14.615h4.068z"
											>
												<animateTransform
													attributeName="transform"
													attributeType="xml"
													dur="0.6s"
													from="0 25 25"
													repeatCount="indefinite"
													to="360 25 25"
													type="rotate"
												/>
											</path>
										</svg>
										{name}
										<progress
											class="progress progress-primary inline"
											value={finished[0]}
											max={finished[1]}
										/>
									{/if}
								</li>
							{/each}
						</ul>
					</div>
				{/if}
			{:else}
				<svg
					class="h-1/3 w-1/3 inline"
					enable-background="new 0 0 50 50"
					version="1.1"
					viewBox="0 0 50 50"
					xml:space="preserve"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="m43.935 25.145c0-10.318-8.364-18.683-18.683-18.683-10.318 0-18.683 8.365-18.683 18.683h4.068c0-8.071 6.543-14.615 14.615-14.615s14.615 6.543 14.615 14.615h4.068z"
					>
						<animateTransform
							attributeName="transform"
							attributeType="xml"
							dur="0.6s"
							from="0 25 25"
							repeatCount="indefinite"
							to="360 25 25"
							type="rotate"
						/>
					</path>
				</svg>
			{/if}
		{/await}
	</div>
</div>
