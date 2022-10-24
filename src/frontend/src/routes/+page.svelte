<!-- TODO: Better layout that includes more info and PowerSchool stuff -->
<script>
	import exampleSrc from '$lib/assets/calendar.jpeg';
	import powerschoolSrc from '$lib/assets/powerschool.png';
</script>

<main class="overflow-x-hidden">
	<section class="h-screen" id="home">
		<div class="hero h-full">
			<div class="hero-content flex-col">
				<h1 class="text-5xl">Skootils</h1>
				<p>Many utilities...</p>
				<div class="flex flex-row justify-center space-x-3 w-screen">
					<!-- <div class="tooltip tooltip-bottom tooltip-primary" data-tip="hello"> -->
					<a class="btn btn-primary" href="/learnatvcs">Scrape learn@vcs</a>
					<!-- </div> -->
					<a class="btn btn-primary" href="/powerschool">PowerSchool++</a>
					<a class="btn btn-primary" href="/email">Mail-tilities</a>
					<a class="btn btn-error" href="/wipe">Wipe storage</a>
				</div>
			</div>
		</div>
	</section>

	<section class="h-screen bg-cyan-300 dark:bg-blue-900" id="learnatvcs">
		<div class="hero h-full">
			<div class="hero-content flex-row">
				<img src={exampleSrc} alt="Example" class="mx-3 rounded-lg shadow-2xl max-w-[50%]" />
				<div>
					<h1 class="text-4xl">learn@vcs scraper</h1>
					<p class="py-6">
						Teachers, sooner or later they say, <b
							>will stop providing calendar events of their assignments' due dates</b
						>
						(see left).<br />
						As a person who uses said calendar, this means I needed to make that calendar myself. But
						as a programmer, I didn't want to do this manually. Instead, I wanted to automate this process.
					</p>
					<a href="/learnatvcs" class="m-auto btn btn-primary">Get Started</a>
				</div>
			</div>
		</div>
	</section>
	<section class="h-screen bg-green-300 dark:bg-emerald-800" id="powerschool">
		<div class="hero h-full">
			<div class="hero-content flex-row">
				<div>
					<h1 class="text-4xl">PowerSchool++</h1>
					<p class="py-6">
						The better grade viewing experience. Track your history and experiment with future
						assignments.
					</p>
					<a href="/powerschool" class="m-auto btn btn-primary">Get Started</a>
				</div>
				<img src={powerschoolSrc} class="rounded-lg max-w-[50%]" alt="Demo" />
			</div>
		</div>
	</section>
	<footer class="footer p-10 bg-neutral text-neutral-content">
		<div class="w-1/2">
			<p>
				Bryan Hu (@ThatXliner). <br /><br /> <b>Logo credits:</b> The logo's pencil is from
				>Twitter's open source
				<a href="https://twemoji.twitter.com" target="_blank" rel="noreferrer" class="link"
					>Twemoji emoji set</a
				>
				(License: CC BY-SA 4.0). I have only rotated the pencil and added the S.
			</p>
		</div>
		<div class="w-3/4">
			<span class="footer-title">Social</span>
			<div class="grid grid-flow-col gap-4">
				<a
					href="https://github.com/ThatXliner/skootils"
					target="_blank"
					rel="noreferrer"
					title="Github Repository"
					><svg
						xmlns="http://www.w3.org/2000/svg"
						aria-hidden="true"
						role="img"
						class="h-6 w-6 inline"
						preserveAspectRatio="xMidYMid meet"
						viewBox="0 0 24 24"
						><path
							fill="currentColor"
							fill-rule="evenodd"
							d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385c.6.105.825-.255.825-.57c0-.285-.015-1.23-.015-2.235c-3.015.555-3.795-.735-4.035-1.41c-.135-.345-.72-1.41-1.23-1.695c-.42-.225-1.02-.78-.015-.795c.945-.015 1.62.87 1.845 1.23c1.08 1.815 2.805 1.305 3.495.99c.105-.78.42-1.305.765-1.605c-2.67-.3-5.46-1.335-5.46-5.925c0-1.305.465-2.385 1.23-3.225c-.12-.3-.54-1.53.12-3.18c0 0 1.005-.315 3.3 1.23c.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23c.66 1.65.24 2.88.12 3.18c.765.84 1.23 1.905 1.23 3.225c0 4.605-2.805 5.625-5.475 5.925c.435.375.81 1.095.81 2.22c0 1.605-.015 2.895-.015 3.3c0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12Z"
							clip-rule="evenodd"
						/></svg
					></a
				>
			</div>
		</div>
	</footer>
</main>
