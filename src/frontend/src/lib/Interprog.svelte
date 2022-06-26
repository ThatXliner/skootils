<script lang="ts">
	export let input: {
		name: string;
		progress: boolean | null | [number, number] | string;
	}[];
</script>

{#each input as task (task.name)}
	{@const progress = task['progress']}
	{@const name = task['name']}
	{#if progress === true}
		<slot name="finished" task={name}
			><li>
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
			</li></slot
		>
	{:else if progress === false || (Array.isArray(progress) && progress[1] === -1)}
		<slot name="queued" task={name}
			><li>
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
			</li></slot
		>
	{:else if typeof progress === 'string'}
		<slot name="errored" task={name} error={progress}
			><li>
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
				<s>{name}</s>
				<span class="ml-2 p-2 bg-error text-error-content rounded-box">{name}</span>
			</li></slot
		>
	{:else if Array.isArray(progress) && typeof progress[0] === 'number'}
		<!-- No "if progress[0] == progress[1]" because
        that's supposed to be handled by the sender -->
		<slot name="bar" task={name} finished={progress[0]} total={progress[1]}
			><li>
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
					class="progress progress-primary w-56 h-4"
					value={progress[0]}
					max={progress[1]}
				/>
			</li></slot
		>
	{:else if progress === null}
		<slot name="spinner" task={name}
			><li>
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
			</li></slot
		>
	{/if}
{/each}
