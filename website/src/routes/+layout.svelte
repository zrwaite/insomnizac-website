<script>
	import Nav from "../lib/nav.svelte"
</script>

<svelte:head>
  <link href="https://fonts.googleapis.com/css?family=Roboto+Mono" rel="stylesheet">
</svelte:head>

<style global lang="scss">
    body,html,#container {
		min-height:100vh;
		overflow:scroll;
	}
	body>div {
		overflow:scroll;
	}
	#pageContainer {
		display: flex;
		flex-direction: row;
		height: 100vh;
		background-color: color('background');
		overflow:scroll;
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

</style> 
<section id="pageContainer">
	<Nav></Nav>
	<div id="contentContainer">
		<main>
			<slot></slot>
		</main>
		<!-- <footer>Zac Waite</footer> -->
	</div>
</section>