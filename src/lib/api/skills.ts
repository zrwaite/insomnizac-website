import type { SkillType } from '$lib/types'
import { pool } from './database'
import { browser } from '$app/environment'
import { error } from '@sveltejs/kit'

export const getSkills = async (): Promise<SkillType[]> => {
	if (browser) throw error(400, 'Ran on client')
	const res = await pool.query('SELECT * FROM skills')
	const skills: SkillType[] = []
	res.rows.forEach((row) => {
		skills.push({
			...row,
			created_at: row.created_at.toISOString(),
			updated_at: row.updated_at.toISOString()
		})
	})
	return skills
}
