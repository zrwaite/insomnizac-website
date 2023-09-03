import type { ProjectsData } from '$lib/types'

export async function load({ data }: { data: ProjectsData }): Promise<ProjectsData> {
	return data
}
