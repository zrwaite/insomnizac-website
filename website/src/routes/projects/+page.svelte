<script lang="ts">
	import HomepageProjectPanel from '$lib/components/ProjectPanel/HomepageProjectPanel.svelte';
	import TitleRow from '$lib/components/TitleRow.svelte';
	import type { ProjectsData } from './+page';

	export let data: ProjectsData;
	$: projects = data.projects;

</script>

<article>
	<TitleRow title={'Projects'} />
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
