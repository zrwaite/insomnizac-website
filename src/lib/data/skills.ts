import type { SkillType } from '$lib/types'
import { poolOptions } from './database'
import { browser } from '$app/environment'

export const getSkills = async (): Promise<SkillType[]> => {
	if (browser) return []
	const pg = await import('pg')
	const pool = new pg.Pool(poolOptions)
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
