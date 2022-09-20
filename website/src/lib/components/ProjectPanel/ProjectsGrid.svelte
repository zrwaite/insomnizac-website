<script lang="ts">
	import type { ProjectType } from "$lib/types";
	import HomepageProjectPanel from "./HomepageProjectPanel.svelte";
	import ProjectPanel from "./ProjectPanel.svelte";
	export let projects : ProjectType[]
	export let panelType: "homepage" | "project"
</script>

<div class="projects">
	{#each projects as project}
		{#if panelType === "homepage"}
			<HomepageProjectPanel project={project} />
		{:else if panelType === "project"}
			<ProjectPanel project={project} />
		{/if}
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