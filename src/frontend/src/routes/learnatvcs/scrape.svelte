<script lang="ts">
	/// Status: all done, a bit more surrounding styling
	/// could be done. Need to cache (although that could be done in backend)
	/// and keep command in store? prob not
	import { Command, type Child } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';

	let command;
	let child: Promise<Child>;
	let tasks: { finished: boolean | null | [number, number]; name: string }[] = [];

	onMount(() => {
		window.sessionStorage.removeItem('output'); // Don't use old data when failed
		command = new Command('learnatvcs', [
			'-um',
			'learnatvcs',
			window.sessionStorage.getItem('config') ?? 'null'
		]);
		command.on('close', (data) => {
			console.log(`command finished with code ${data.code} and signal ${data.signal}`);
			if (window.sessionStorage.getItem('output') !== null) {
				window.location.assign('/learnatvcs/results');
			}
		});
		command.on('error', (error) => console.error(`command error: "${error}"`));
		command.stdout.on('data', (line) => {
			console.log('Got data: ', line);
			let input = JSON.parse(line);
			if (Array.isArray(input)) {
				tasks = input;
			} else {
				window.sessionStorage.setItem('output', line);
			}
		});
		command.stderr.on('data', (line) => console.log(`command stderr: "${line}"`));
		child = command.spawn();
		return () => {
			(async function () {
				await (await child).kill();
			})();
		};
	});
</script>

{#if child != null}
	{#await child then}
		<ul class="text-2xl m-3">
			{#each tasks as task (task.name)}
				{#if task['finished'] === true}
					<li>
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
						{task['name']}
					</li>
				{:else if task['finished'] === false || (Array.isArray(task['finished']) && task['finished'][0] == 0)}
					<li>
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
						{task['name']}
					</li>
				{:else if typeof task['finished'] === 'string'}
					<li>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-12 w-12 fill-error inline"
							viewBox="0 0 20 20"
							fill="currentColor"
						>
							<path
								fill-rule="evenodd"
								d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
								clip-rule="evenodd"
							/>
						</svg>
						<s>{task['name']}</s>
						<span class="ml-2 p-2 bg-error text-error-content rounded-box">{task['finished']}</span>
					</li>
				{:else if task['finished'] === null}
					<li>
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
						{task['name']}
					</li>
				{:else}
					<li>
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
						{task['name']}
						<progress
							class="progress progress-primary w-56 h-4"
							value={task['finished'][0]}
							max={task['finished'][1]}
						/>
					</li>
				{/if}
			{/each}
		</ul>
	{/await}
{/if}
