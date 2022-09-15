import { error } from '@sveltejs/kit';
import type { ProjectType } from '$lib/types';
import { graphql } from '$lib/data/graphql';
import { PROJECTS_QUERY } from '$lib/data/queries/projects';

export interface HomeData {	
	projects: ProjectType[];
}

export async function load(): Promise<HomeData> {
	try {
		const data = await graphql.request(PROJECTS_QUERY);
		return data;
	} catch (e) {
		console.log(e);
		throw error(400, 'Request failed');
	}
}
