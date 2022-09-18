<script lang="ts">
	export let onClick: () => void;
</script>

<div class="container" on:click={onClick}>
	<slot />
</div>

<style lang="scss">
	.container {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		border: 0.2rem solid color('primary');
		border-radius: 0.5rem;
		cursor: pointer;
		padding: 0.5rem;
		transition: background-color 0.15s ease-in-out;
		&:hover {
			background-color: rgba(color('primary'), 0.2);
		}
	}
</style>