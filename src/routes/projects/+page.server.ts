import { error } from '@sveltejs/kit'
import type { ProjectsData } from '$lib/types'
import { getProjects } from '$lib/data/projects'
import { getSkills } from '$lib/data/skills'

export async function load(): Promise<ProjectsData> {
	try {
		const skills = await getSkills()
		const projects = await getProjects(skills)
		return {
			projects
		}
	} catch (e) {
		console.log(e)
		throw error(400, 'Request failed')
	}
}
