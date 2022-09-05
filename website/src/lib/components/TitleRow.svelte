<script lang="ts">
	export let title: string;
	let width = `${title.length * 1.2}rem`;
</script>

<div class="titleRow" style="--width:{width}">
	<h2>{title}</h2>
	<hr />
</div>

<style lang="scss">
	.titleRow {
		width: 100%;
		display: flex;
		align-items: center;
		margin-bottom: 1rem;
		h2 {
			font-size: 2rem;
			min-width: m#{i}n(var(--width), 80vw);
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
