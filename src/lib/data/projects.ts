import { browser } from '$app/environment'
import type { ProjectType, SkillType } from '$lib/types'
import { poolOptions } from './database'
import { getRepositoriesData } from './github'

export const getProjects = async (skills: SkillType[]): Promise<ProjectType[]> => {
	if (browser) return []
	const pg = await import('pg')
	const pool = new pg.Pool(poolOptions)

	const res = await pool.query('SELECT * FROM projects')
	const projects: ProjectType[] = []
	res.rows.forEach((row) => {
		const project = {
			...row,
			created_at: row.created_at.toISOString(),
			updated_at: row.updated_at.toISOString(),
			skills: skills.filter((skill) => row.skill_ids.includes(skill.id))
		}
		projects.push(project)
	})
	return getRepositoriesData(projects)
}
