<script lang="ts">
	import ProjectsGrid from '$lib/components/ProjectPanel/ProjectsGrid.svelte'
	import TitleRow from '$lib/components/TitleRow.svelte'
	import type { ProjectsData } from '$lib/types'

	export let data: ProjectsData
	$: projects = data.projects

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
