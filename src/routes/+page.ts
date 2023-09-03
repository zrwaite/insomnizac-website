import type { HomeData } from '$lib/types'
import { error } from '@sveltejs/kit'
import { PUBLIC_ENDPOINT } from '$env/static/public'

export async function load(): Promise<HomeData> {
	try {
		const skills = await (await fetch(PUBLIC_ENDPOINT + '/api/skills', { method: 'GET' })).json()
		const projects = await (await fetch(PUBLIC_ENDPOINT + '/api/projects', { method: 'GET' })).json()
		return {
			projects,
			skills
		}
	} catch (e) {
		console.log(e)
		throw error(400, 'Request failed')
	}
}
