<script lang="ts">
	import { error } from '@sveltejs/kit';
	import { graphql } from '../../data/graphql';
	import { gql } from 'graphql-request';

	import type { ProjectType } from 'src/types';

	const PROJECTS_QUERY = gql`
		query {
			projects {
				name
				slug
				description
				githubName
				devpostLink
				projectLink
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
		} catch (e) {
			throw error(400, 'Request failed');
		}
	};
	loadProjects();
</script>

<article>
	<h1>Projects</h1>
	{#if projectsLoaded}
		{#each projects as project}
			<p>{project.name}</p>
		{/each}
	{:else}
		<p>Loading...</p>
	{/if}
</article>
