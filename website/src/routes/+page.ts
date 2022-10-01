import { error } from '@sveltejs/kit';
import type { ProjectType, SkillType } from '$lib/types';
import { graphql } from '$lib/data/graphql';
import { PROJECTS_QUERY } from '$lib/data/queries/projects';
import { SKILLS_QUERY } from '$lib/data/queries/skills';

export interface HomeData {	
	projects: ProjectType[]
	skills: SkillType[]
}

export const defaultHomeData: HomeData = {
	projects: [],
	skills: []
};

export async function load(): Promise<HomeData> {
	try {
		const projectsData = await graphql.request(PROJECTS_QUERY);
		const skillsData = await graphql.request(SKILLS_QUERY);
		return {
			projects: projectsData.projects,
			skills: skillsData.skills	
		};
	} catch (e) {
		console.log(e);
		throw error(400, 'Request failed');
	}
}
