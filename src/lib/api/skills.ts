import type { SkillType } from '$lib/types'
import { pool } from './database'
import { browser } from '$app/environment'
import { error } from '@sveltejs/kit'
import { redis } from './redis'

export const getSkills = async (): Promise<SkillType[]> => {
	if (browser) throw error(400, 'Ran on client')
	const skillCache = await redis.get('insomnizac-website-skills')
	if (skillCache) {
		return JSON.parse(skillCache)
	}
	const res = await pool.query('SELECT * FROM skills')
	const skills: SkillType[] = []
	res.rows.forEach((row) => {
		skills.push({
			...row,
			created_at: row.created_at.toISOString(),
			updated_at: row.updated_at.toISOString()
		})
	})
	redis.set('insomnizac-website-skills', JSON.stringify(skills), { EX: 60 * 60 * 24 * 7 })
	return skills
}
