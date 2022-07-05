<script lang="ts">
	import HomeButton from '$lib/HomeButton.svelte';
	import AlertError from '$lib/AlertError.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	let emailType: 'late' | 'missing' = 'late';
	let getTeachers: Promise<{
		[key: string]: { class_name: string; email: string; period: string };
	}> = invoke('get_teachers');
	let teachers: { [key: string]: { class_name: string; email: string; period: string } } = {};
	getTeachers.then((out) => {
		teachers = out;
	});
	let selectedTeacher: string;
	let assignmentName: string;
	let copied: boolean = false;

	type Grade = '6th' | '7th' | '8th' | 'Freshman' | 'Sophomore' | 'Junior' | 'Senior';
	let getAuthorInfo: Promise<{ name: string; grade: Grade }> = invoke('get_user_info');
	let hasInfo: boolean = false;
	let authorGrade: Grade;
	let authorName: string;
	$: formValid = selectedTeacher !== 'Choose a teacher' && assignmentName && authorName;
	getAuthorInfo.then((out) => {
		hasInfo = true;
		authorName = out['name'];
		authorGrade = out['grade'];
	});
	$: generatedEmail =
		`Dear ${selectedTeacher},\n` +
		`I am emailing you about a ${emailType} assignment of mine. ` +
		`The name of this assigment is '${assignmentName}' and ` +
		`I have now ${emailType == 'late' ? 're-turned' : 'turned'} it in. ` +
		`May you please ${emailType == 'late' ? 're-grade' : 'grade'} it?\n\n` +
		'Sincerely,\n' +
		`${authorName}, ${authorGrade}\n\n` +
		`${teachers[selectedTeacher]?.class_name} - ${teachers[selectedTeacher]?.period?.replaceAll(
			/\(|\)/g,
			''
		)}`; // TODO: due date
</script>

{#if !hasInfo}
	<div class="modal modal-open">
		<div class="modal-box">
			<h3 class="font-bold text-lg">Please enter your student information</h3>
			<div class="form-control">
				<span class="label label-text">Full name</span>
				<input
					type="text"
					bind:value={authorName}
					placeholder="Bryan Hu"
					class="input input-bordered"
				/>
			</div>
			<div class="form-control">
				<span class="label label-text">Grade</span>
				<select class="select select-bordered w-full max-w-xs" bind:value={authorGrade}>
					<option disabled selected>Select your grade</option>
					<option>6th</option>
					<option>7th</option>
					<option>8th</option>
					<option>Freshman</option>
					<option>Sophomore</option>
					<option>Junior</option>
					<option>Senior</option>
				</select>
			</div>
			<div class="modal-action">
				<button
					class="btn btn-primary"
					class:btn-disabled={!authorName || authorGrade === 'Select your grade'}
					on:click={() => {
						invoke('set_user_info', { to: { name: authorName, grade: authorGrade } }).then(() => {
							hasInfo = true;
						});
					}}>Submit</button
				>
			</div>
		</div>
	</div>
{/if}
<main class="pl-3 bg-base-200 min-h-screen">
	<HomeButton extraClasses="my-2" />
	<h1 class="font-bold text-3xl">Email Generator &amp; Sender</h1>
	<p class="py-3 w-sm mx-auto">
		The ultimate tool for yeeting emails at teachers at ease. No more forgetting saying who you are
		or appearing too rude to your teacher with our carefully crafted email templates!
	</p>
	<div class="flex space-x-4 justify-evenly">
		<div class="card w-md shadow-2xl bg-base-100 h-fit">
			<div class="card-body">
				<div class="alert shadow-lg bg-blue-200 dark:bg-blue-800 ml-2 w-64">
					I want to write an email for a ...
				</div>
				<div class="form-control w-fit ml-2 mt-3">
					<label class="label cursor-pointer">
						<span class="label-text text-lg pr-3">Late assignment</span>
						<input
							type="radio"
							bind:group={emailType}
							class="radio radio-lg radio-primary"
							value="late"
						/>
					</label>
					<label class="label cursor-pointer">
						<span class="label-text text-lg pr-3">Missing assignment</span>
						<input
							type="radio"
							bind:group={emailType}
							class="radio radio-lg radio-primary"
							value="missing"
						/>
					</label>
				</div>
			</div>
		</div>

		<div class="card w-md shadow-2xl bg-base-100">
			<div class="card-body">
				<div class="form-control">
					<span class="label label-text">Assignment name</span>
					<input
						bind:value={assignmentName}
						type="text"
						placeholder="Castle Minecraft Project"
						class="input input-bordered"
					/>
				</div>

				<div class="form-control">
					<span class="label label-text">Teacher to email</span>
					{#await getTeachers then teachers}
						<select class="select select-bordered w-full max-w-xs" bind:value={selectedTeacher}>
							<option disabled selected>Choose a teacher</option>
							{#each Object.keys(teachers) as teacher}
								<option>{teacher}</option>
							{/each}
						</select>
					{:catch error}
						<AlertError message={error} />
					{/await}
				</div>
				<div class="form-control">
					<button
						class="link link-secondary"
						on:click={() => {
							hasInfo = false;
						}}>Update student info</button
					>
				</div>
			</div>
		</div>
	</div>
	<div class="w-fit card mx-auto shadow-2xl bg-base-100 my-3">
		<div class="card-body">
			<div class="card-actions justify-center">
				<div
					class="tooltip-left tooltip-success"
					class:tooltip-open={copied}
					class:tooltip={copied}
					data-tip="Copied!"
				>
					<button
						class="btn btn-primary"
						class:btn-disabled={!formValid}
						class:btn-success={copied}
						on:click={() => {
							navigator.clipboard.writeText(generatedEmail).then(() => {
								copied = true;
							});

							window.setTimeout(() => {
								copied = false;
							}, 1000);
						}}>Copy</button
					>
				</div>
				<a
					class="btn btn-primary"
					class:btn-disabled={!formValid}
					href="mailto:{teachers[selectedTeacher]?.email}?subject={emailType[0].toUpperCase() +
						emailType.slice(1)}%20Assignment&body={encodeURIComponent(generatedEmail)}">Send</a
				>
			</div>
			{#if formValid}
				<articlw class="max-w-sm break-words bg-base-200 p-3 rounded-box mx-auto"
					>{@html generatedEmail.replaceAll('\n', '<br/>')}</articlw
				>
			{:else}
				<AlertError message="Please complete the form" />
			{/if}
		</div>
	</div>
</main>

<!-- TODO: Support multiple agendas in one email (so use checkbox instead of radio) -->
