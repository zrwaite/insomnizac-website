<script lang="ts">
	export let title: string
	export let variant: 'h1'|'h2' = 'h2'
	let scale = variant === 'h1' ? 1.8 : 1.2
	let width = `${title.length * scale}rem`
</script>

<div class="titleRow" style="--width:{width}">
	{#if variant === 'h1'} 
		<h1>{title}</h1>
	{:else if variant === 'h2'} 
		<h2>{title}</h2>
	{/if}
	<hr />
</div>

<style lang="scss">
	.titleRow {
		width: 100%;
		display: flex;
		align-items: center;
		margin-bottom: 1rem;
		h1 {
			font-size: 3rem;
			min-width: Min(var(--width), calc(80vw - 4rem));
		}
		h2 {
			font-size: 2rem;
			min-width: Min(var(--width), calc(80vw - 4rem));
		}
		hr {
			margin: 0 1rem;
			width: 100%;
			height: 0;
			border-left: 0;
			border-right: 0;
		}
	}
</style>
