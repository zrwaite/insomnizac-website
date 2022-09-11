<script lang="ts">
	import HomepageProjectPanel from '$lib/components/ProjectPanel/HomepageProjectPanel.svelte';
	import type { ProjectsData } from './+page';

	export let data: ProjectsData;
	$: projects = data.projects;
	console.log(data);

</script>

<article>
	<h1>Projects</h1>
	<div class="projects">
		{#each projects as project}
			<HomepageProjectPanel {project} />
		{/each}
	</div>
</article>

<style lang="scss">
	.projects {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		flex-wrap: wrap;
	}
</style>
