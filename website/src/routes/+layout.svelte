<script>
	import Nav from '$lib/components/nav/Nav.svelte';
</script>

<section id="pageContainer">
	<div id="contentContainer">
		<main>
			<slot />
		</main>
		<!-- <footer>Zac Waite</footer> -->
	</div>
	<Nav />
</section>

<style global lang="scss">
	body,
	html,
	#container {
		min-height: 100vh;
		overflow: scroll;
	}
	body > div {
		overflow: scroll;
	}
	#pageContainer {
		display: flex;
		flex-direction: row-reverse;
		height: 100vh;
		background-color: color('background');
		overflow: scroll;
	}
	#contentContainer {
		width: 100%;
		height: 100%;
	}
	main {
		min-height: 95%;
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
			width: 100%;
			height: auto;
			padding-bottom: 4rem;
		}
	}
</style>
