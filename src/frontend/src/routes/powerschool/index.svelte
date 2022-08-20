<!-- @hmr:keep-all -->
<script lang="ts">
	import AccountInput from '$lib/AccountInput.svelte';
	import QuarterInput from '$lib/QuarterInput.svelte';
	import HomeButton from '$lib/HomeButton.svelte';
	import Interprog from '$lib/Interprog.svelte';

	import { Command, type Child } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';

	let command;
	let child: Promise<Child>;
	let hasCredentials: boolean | null = null;
	let progress: {
		name: string;
		progress: boolean | null | [number, number] | string;
	}[];
	let quarterChoices: string[];
	let error: string | null = null;

	onMount(() => {
		window.sessionStorage.removeItem('output');
		command = Command.sidecar('../../powerschool/dist/powerschool/powerschool');
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
				if (typeof input[0] === 'string') {
					quarterChoices = input;
				} else {
					progress = input;
				}
			} else {
				if ('error' in input) {
					error = input['error'];
					return;
				}
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

<input type="checkbox" id="my-modal" class="modal-toggle" />
<div class="modal" class:modal-open={error !== null}>
	<div class="modal-box">
		<h3 class="font-bold text-lg">Error!</h3>
		<div class="alert alert-error shadow-lg">
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
						d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
					/></svg
				>
				<span>{error}</span>
			</div>
		</div>

		<div class="modal-action">
			<a href="/" class="btn btn-primary">Bring me back home</a>
		</div>
	</div>
</div>
<div class="hero min-h-screen bg-base-200">
	<HomeButton extraClasses="mr-auto mb-auto ml-3 mt-2" />
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
						<ul class="text-2xl">
							<Interprog input={progress} />
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
