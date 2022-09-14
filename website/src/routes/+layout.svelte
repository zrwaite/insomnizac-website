<script>
	import Nav from '$lib/components/nav/Nav.svelte';
</script>

<section id="pageContainer">
	<Nav />
	<div id="contentContainer">
		<main>
			<slot />
		</main>
		<!-- <footer>Zac Waite</footer> -->
	</div>
</section>

<style global lang="scss">
	body,
	html,
	#container {
		min-height: 100vh;
		background-color: color('background');
	}
	#pageContainer {
		display: flex;
		flex-direction: row;
		height: 100vh;
	}
	#contentContainer {
		width: 100%;
		margin-left: 5rem;
	}
	main {
		min-height: 95%;
		width: 100%;
	}
	footer {
		position: absolute;
		bottom: 0;
		width: calc(100% - 4rem);
		height: auto;
	}
	@media screen and (max-width: 30rem) {
		#pageContainer {
			flex-direction: column;
		}
		#contentContainer {
			margin-left: 0;
			margin-top: 5rem;
			display: flex;
			flex-direction: row;
			justify-content: center;
			align-items: center;
			flex-wrap: wrap;
			padding: 0 0.5rem;
			width: calc(100% - 1rem);
		}
	}
</style>
