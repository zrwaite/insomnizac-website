import { getProjects } from '$lib/api/projects'

export async function GET(): Promise<Response> {
	const projects = await getProjects()
	return new Response(JSON.stringify(projects), {
		status: 200,
		headers: { 'Content-Type': 'application/json' }
	})
}
