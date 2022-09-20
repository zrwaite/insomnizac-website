<script lang="ts">
	import HomepageProjectPanel from '$lib/components/ProjectPanel/HomepageProjectPanel.svelte';
	import ProjectsGrid from '$lib/components/ProjectPanel/ProjectsGrid.svelte';
	import TitleRow from '$lib/components/TitleRow.svelte';
	import type { ProjectsData } from './+page';

	export let data: ProjectsData;
	$: projects = data.projects;

</script>

<article>
	<div class="projectsHeader">
		<TitleRow title={'Projects'} variant={'h1'}/>
	</div>
	<ProjectsGrid projects={projects} panelType={"project"} />
</article>

<style lang="scss">
	.projectsHeader {
		margin: 1rem;
	}
</style>
