<script>
	import Nav from '$lib/components/nav/Nav.svelte';
</script>

<svelte:head>
	<link
		href="https://fonts.googleapis.com/css2?family=Roboto+Mono:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;1,100;1,200;1,300;1,400;1,500;1,600;1,700&display=swap"
		rel="stylesheet"
	/>
</svelte:head>
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
