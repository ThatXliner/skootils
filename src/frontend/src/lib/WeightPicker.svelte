<script lang="ts">
	import { onMount } from 'svelte';
	import AlertWarning from './AlertWarning.svelte';

	export let categories: string[];
	let _categories: Set<string> = new Set();
	onMount(() => {
		_categories = new Set(categories);
	});
	let name: string;
	let weight: number;
	export let weights: { [key: string]: number } | null;
	let _weights: { [key: string]: number } = {};
	$: if (_categories.size > 0) {
		// unweighted
		if (Object.keys(_weights).length == 0) {
			weights = null;
		}
		// TODO: auto calc
		else {
			weights = _weights;
			// weights = Object.fromEntries(categories.map((e) => [e, 1]));
		}
	} else {
		weights = _weights;
	}
</script>

<div class="flex justify-evenly space-x-2">
	{#if _categories.size != 0}
		<div class="bg-base-300 rounded-box p-3 w-fit mx-auto">
			<h2 class="text-lg font-bold mb-1">Specify weights</h2>
			<div class="form-control w-full">
				<label class="label" for="name"><span class="label-text">Assignment type</span></label>
				<select class="select select-bordered" bind:value={name}>
					<option disabled selected>Pick one</option>
					{#each [..._categories] as category}
						<option>{category}</option>
					{/each}
				</select>
			</div>
			<div class="form-control w-full">
				<label class="label" for="name"
					><span class="label-text">Weight value</span><span class="label-text-alt"
						>in decimal format</span
					></label
				>
				<input type="number" bind:value={weight} class="rounded bg-base-200 p-2 text-lg" />
			</div>

			<button
				class="mt-3 btn btn-primary"
				on:click={() => {
					_weights[name] = weight;
					_categories.delete(name);
					_categories = _categories;
					name = 'Pick one';
					weight = 0;
				}}
				disabled={name == 'Pick one' || !weight}>Add</button
			>
		</div>
	{/if}
	<div class="flex flex-col justify-evenly">
		{#if Object.keys(_weights).length > 0}
			<ul class="menu bg-base-200 w-56 h-full rounded-box justify-center">
				{#each Object.entries(_weights) as [name, value]}
					<li
						class="rounded-none"
						on:click={() => {
							delete _weights[name];
							_categories = _categories.add(name);
							_weights = _weights;
						}}
					>
						<span
							><svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 20 20"
								fill="currentColor"
								class="w-5 h-5 inline"
							>
								<path
									fill-rule="evenodd"
									d="M8.75 1A2.75 2.75 0 006 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 10.23 1.482l.149-.022.841 10.518A2.75 2.75 0 007.596 19h4.807a2.75 2.75 0 002.742-2.53l.841-10.52.149.023a.75.75 0 00.23-1.482A41.03 41.03 0 0014 4.193V3.75A2.75 2.75 0 0011.25 1h-2.5zM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4zM8.58 7.72a.75.75 0 00-1.5.06l.3 7.5a.75.75 0 101.5-.06l-.3-7.5zm4.34.06a.75.75 0 10-1.5-.06l-.3 7.5a.75.75 0 101.5.06l.3-7.5z"
									clip-rule="evenodd"
								/>
							</svg>
							{name}: {value * 100}%</span
						>
					</li>
				{/each}
			</ul>
			{#if _categories.size > 0}
				<AlertWarning
					message="Only some weights provided, <b>auto-calcuating the rest</b>"
					raw={true}
				/>
			{/if}
		{:else}
			<AlertWarning
				message="No weights provided.<br/>Using <b>unweighted calculation</b>"
				raw={true}
			/>
		{/if}
	</div>
</div>
