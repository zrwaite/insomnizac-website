import { error } from '@sveltejs/kit'
import type { HomeData } from '$lib/types'
import { getProjects } from '$lib/data/projects'
import { getSkills } from '$lib/data/skills'

export async function load(): Promise<HomeData> {
	try {
		const skills = await getSkills()
		const projects = await getProjects(skills)
		return {
			projects,
			skills
		}
	} catch (e) {
		console.log(e)
		throw error(400, 'Request failed')
	}
}
