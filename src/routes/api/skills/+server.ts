import { getSkills } from '$lib/api/skills'

export async function GET(): Promise<Response> {
	const skills = await getSkills()
	return new Response(JSON.stringify(skills), {
		status: 200,
		headers: { 'Content-Type': 'application/json' }
	})
}
