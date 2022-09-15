<script lang="ts">
	import type { ProjectType } from "$lib/types";
	import HomepageProjectPanel from "./HomepageProjectPanel.svelte";

	export let projects : ProjectType[]
</script>

<div class="projects">
	{#each projects as project}
		<HomepageProjectPanel {project} />
	{/each}
</div>

<style lang="scss">
	.projects {
		display: grid;
		width: 100%;
		grid-template-columns: repeat(auto-fill, minmax(25rem, 1fr));
		justify-items: center;
	}
	@media screen and (max-width: 40rem) {
		.projects {
			display: flex;
			flex-direction: column;
		}
	}
</style>