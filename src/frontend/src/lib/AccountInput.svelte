<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	let dispatch = createEventDispatcher();

	let username = '';
	let password = '';
	let _showPassword = false;
</script>

<div class="card w-full max-w-sm shadow-2xl bg-base-100">
	<div class="card-body">
		<div>
			<div class="form-control">
				<label class="label">
					Username
					<input
						type="text"
						placeholder="first.last"
						class="input input-bordered"
						bind:value={username}
					/>
				</label>
				<label class="label">
					Password
					<input
						type={_showPassword ? 'text' : 'password'}
						placeholder="ThreeRandomWords69"
						class="input input-bordered"
						on:input={({ target }) => {
							password = target.value;
						}}
					/></label
				>

				<label class="label cursor-pointer">
					<span class="label-text">Show password</span>
					<input type="checkbox" class="toggle" bind:checked={_showPassword} />
				</label>
			</div>
		</div>

		<div class="form-control mt-6">
			<button
				class="btn btn-primary"
				on:click={() => {
					dispatch('input', { username, password });
				}}
				class:btn-disabled={username === '' || password === ''}>Login</button
			>
		</div>
	</div>
</div>
