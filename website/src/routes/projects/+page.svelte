<script lang="ts">
	import { error } from '@sveltejs/kit';
	import { graphql } from '../../data/graphql';
	import { gql } from 'graphql-request';

	import type { ProjectType } from 'src/types';
	import HomepageProjectPanel from '$lib/components/ProjectPanel/HomepageProjectPanel.svelte';

	const PROJECTS_QUERY = gql`
		query {
			projects {
				name
				slug
				description
				githubName
				devpostLink
				projectLink
				image
			}
		}
	`;
	let projects: ProjectType[] = [];
	let projectsLoaded = false;
	const loadProjects = async () => {
		await new Promise((resolve) => setTimeout(resolve, 1000));
		try {
			const data = await graphql.request(PROJECTS_QUERY);
			projects = data.projects;
			projectsLoaded = true;
			console.log(projects)
		} catch (e) {
			throw error(400, 'Request failed');
		}
	};
	loadProjects();
</script>

<article>
	<h1>Projects</h1>
	<div class="projects">
		{#if projectsLoaded}
			{#each projects as project}
				<HomepageProjectPanel {project} />
			{/each}
		{:else}
			<p>Loading...</p>
		{/if}
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
