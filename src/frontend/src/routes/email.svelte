<script lang="ts">
	import HomeButton from '$lib/HomeButton.svelte';
	import AlertError from '$lib/AlertError.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	let emailType: 'late' | 'missing' = 'late';
	let getTeachers: Promise<{
		[key: string]: { class_name: string; email: string; period: string };
	}> = invoke('get_teachers');
	let teachers: { [key: string]: { class_name: string; email: string; period: string } } = {};
	$: getTeachers.then((out) => {
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
	$: formValid =
		!(selectedTeacher === 'Choose a teacher' || selectedTeacher === undefined) &&
		assignmentName &&
		authorName;
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
	let _add_teacher_name: string = '';
	let _add_teacher_email: string = '';
	let _add_teacher_period: string = '';
	let _add_teacher_class: string = '';
</script>

<input type="checkbox" id="add-teachers" class="modal-toggle" />
<div class="modal">
	<div class="modal-box">
		<h3 class="font-bold text-lg">Update teacher list</h3>
		{#await getTeachers then teachers}
			<p>You may manually edit your list of teachers below.</p>
			<div class="rounded-box shadow-lg space-x-4 p-2 border-2 flex flex-wrap">
				{#each Object.keys(teachers) as teacher}
					<button
						class="shadow-sm border-2 border-base-200 normal-case btn btn-ghost hover:bg-error hover:text-error-content"
						on:click={() => {
							invoke('remove_teacher', {
								teacherName: teacher
							}).then(() => {
								getTeachers = invoke('get_teachers');
							});
						}}
						>{teacher}
						<sub class="text-3 text-end pl-2">({teachers[teacher]['period']})</sub></button
					>
				{/each}
			</div>
		{:catch}
			<p class="py-4">
				You have not ran the PowerSchool program before. We use that to <b
					>automatically find teachers via your PowerSchool data.</b
				>

				Nonetheless, PowerSchool may be unavailable. Add your teachers below.
			</p>
		{/await}
		<div class="form-control">
			<label class="label">
				<span class="label-text">Teacher name</span>
				<input
					type="text"
					placeholder="Mrs. Rosas"
					bind:value={_add_teacher_name}
					class="input input-bordered"
				/>
			</label>
			<label class="label">
				<span class="label-text">Period</span>
				<input
					type="text"
					placeholder="1A"
					bind:value={_add_teacher_period}
					class="input input-bordered"
				/>
			</label>
			<label class="label">
				<span class="label-text">Class name</span>
				<input
					type="text"
					placeholder="English 7"
					bind:value={_add_teacher_class}
					class="input input-bordered"
				/>
			</label>
			<label class="label">
				<span class="label-text">Email</span>
				<input
					type="email"
					placeholder="crosas@vcs.net"
					bind:value={_add_teacher_email}
					class="input input-bordered"
				/>
			</label><button
				class="btn btn-primary"
				disabled={_add_teacher_email.length == 0 ||
					_add_teacher_name.length == 0 ||
					_add_teacher_class.length == 0 ||
					_add_teacher_period.length == 0}
				on:click={() => {
					invoke('add_teacher', {
						teacherName: _add_teacher_name,
						teacherInfo: {
							email: _add_teacher_email,
							period: _add_teacher_period,
							class_name: _add_teacher_class
						}
					}).then(() => {
						_add_teacher_email = _add_teacher_period = _add_teacher_class = _add_teacher_name = '';
						return (getTeachers = invoke('get_teachers'));
					});
				}}>Add teacher</button
			>
		</div>

		<div class="modal-action">
			<label for="add-teachers" class="btn btn-primary">Done</label>
		</div>
	</div>
</div>
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
				<div class="alert shadow-lg bg-blue-200 dark:bg-blue-800 ml-2 w-64 select-none">
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
						class="powerlink"
						on:click={() => {
							hasInfo = false;
						}}>Update student info</button
					>
				</div>
				<div class="form-control">
					<label class="mx-auto modal-button powerlink" for="add-teachers"
						>Update teacher list</label
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
<style lang="postcss">
	.powerlink {
		@apply link transition-all ease-in-out;
	}
	.powerlink:hover {
		@apply link-accent;
	}
</style>
