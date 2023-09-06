import { browser } from '$app/environment'
import type { ProjectType, SkillType } from '$lib/types'
import { error } from '@sveltejs/kit'
import { pool } from './database'
import { getRepositoriesData } from './github'
import { getSkills } from './skills'
import { redis } from './redis'

export const getProjects = async (skills?: SkillType[]): Promise<ProjectType[]> => {
	if (browser) throw error(400, 'Ran on client')
	const loadedSkills = skills || await getSkills()
	const projectCache = await redis.get('insomnizac-website-projects')
	if (projectCache) {
		return JSON.parse(projectCache)
	}

	const res = await pool.query('SELECT * FROM projects')
	const projects: ProjectType[] = []
	res.rows.forEach((row) => {
		const project = {
			...row,
			created_at: row.created_at.toISOString(),
			updated_at: row.updated_at.toISOString(),
			skills: loadedSkills.filter((skill) => row.skill_ids.includes(skill.id))
		}
		projects.push(project)
	})
	const loadedProjects = await getRepositoriesData(projects)
	await redis.set('insomnizac-website-projects', JSON.stringify(loadedProjects), { EX: 60 * 60 * 24 * 7 })
	return loadedProjects
}
