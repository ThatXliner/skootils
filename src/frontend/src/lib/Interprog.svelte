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
		<slot name="finished" task={name} />
	{:else if progress === false || (Array.isArray(progress) && progress[1] === -1)}
		<slot name="queued" task={name} />
	{:else if typeof progress === 'string'}
		<slot name="errored" task={name} error={progress} />
	{:else if Array.isArray(progress) && typeof progress[0] === 'number'}
		<!-- No "if progress[0] == progress[1]" because
        that's supposed to be handled by the sender -->
		<slot name="bar" task={name} finished={progress[0]} total={progress[1]} />
	{:else if progress === null}
		<slot name="spinner" task={name} />
	{/if}
{/each}
