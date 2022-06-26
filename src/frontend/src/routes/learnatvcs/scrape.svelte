<script lang="ts">
	/// Status: all done, a bit more surrounding styling
	/// could be done. Need to cache (although that could be done in backend)
	/// and keep command in store? prob not
	import { Command, type Child } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';
	import Interprog from '$lib/Interprog.svelte';

	let command;
	let child: Promise<Child>;
	let tasks: {
		name: string;
		progress: boolean | null | [number, number] | string;
	}[] = [];

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
			// XXX: What about onDestroy?
			(async function () {
				await (await child).kill();
			})();
		};
	});
</script>

{#if child != null}
	{#await child then}
		<ul class="text-2xl m-3">
			<Interprog input={tasks} />
		</ul>
	{/await}
{/if}
