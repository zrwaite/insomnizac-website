import { error } from '@sveltejs/kit'
import type { ProjectsData } from '$lib/types'
import { PUBLIC_ENDPOINT } from '$env/static/public'

export async function load(): Promise<ProjectsData> {
	try {
		const projects = await (await fetch(PUBLIC_ENDPOINT + '/api/projects', { method: 'GET' })).json()
		return {
			projects
		}
	} catch (e) {
		console.log(e)
		throw error(400, 'Request failed')
	}
}
