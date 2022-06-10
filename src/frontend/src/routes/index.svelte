<!-- TODO: Better layout that includes more info and PowerSchool stuff -->
<script>
	// But we're doing this for now
	import { open } from '@tauri-apps/api/shell';
</script>

<section class="h-screen" id="home">
	<div class="hero h-[100%]">
		<div class="hero-content flex-col">
			<h1 class="text-5xl">Skootils</h1>
			<p>For the inevitable future... <a href="#what-is-this" class="link">Learn more</a></p>
			<!-- TODO: Split from learn@vcs scraper and powerschool utils -->
			<a class="btn btn-primary" href="/config">Let's do this!</a>
		</div>
	</div>
</section>
<!-- TODO BETTER -->
<!-- <div class="flex justify-center">
	<div class="card w-[80%] bg-base-200 h-[12em] shadow-xl">
		<div class="card-body">
			<h2 class="card-title">About</h2>
			<p>This is a tool built to scrape information on homework and assignments</p>
			<div class="card-actions justify-end">
				<button
					on:click={() => {
						open('https://www.youtube.com/watch?v=xvFZjo5PgG0');
					}}
					class="btn btn-primary">More</button
				>
			</div>
		</div>
	</div>
</div> -->
<section class="h-screen bg-red-300" id="what-is-this">
	<div class="hero h-[100%] flex">
		<h1 class="text-5xl grow-2">What is this?</h1>
		<p class="max-w-md">
			You may or may not have been informed of the thing. Teachers, sooner or later they say, will
			stop providing calendar events of their assignments' due dates
		</p>
	</div>
</section>
