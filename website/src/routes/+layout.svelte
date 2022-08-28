<svelte:head>
  <link href="https://fonts.googleapis.com/css?family=Roboto+Mono" rel="stylesheet">
</svelte:head>

<style global lang="scss">
	
	$colors: (
		primary: #007cd4,
		background: #001201,
		font: white
	);
	@function color($key) {
		@if not map-has-key($colors, $key) {
			@warn "Color `#{$key}` not found.";
		}
		@return map-get($colors, $key);
	}

	* {
		margin: 0;
		padding: 0;
        font-family: 'Roboto Mono';
		color: color('font');
	}
    body,html,#container {
		min-height:100vh;
	}
	#container {
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
		align-items: center;
		height: 100vh;
		background-color: color('background');
	}

</style>

<div id="container"> 
	<slot></slot>
</div>