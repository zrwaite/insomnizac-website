export interface GameType {
	name: string
	slug: string
	src: string
}

export interface ProjectType {
	name: string
	slug: string
	description: string
	github_name: string
	github_link: string
	devpost_link?: string
	project_link?: string
	image: string
	featured: boolean
	skills: SkillType[]
}

export interface SkillType {
	id: string
	name: string
	image: string 
}

export interface HomeData {	
	projects: ProjectType[]
	skills: SkillType[]
}

export const defaultHomeData: HomeData = {
	projects: [],
	skills: []
}

export interface ProjectsData {	
	projects: ProjectType[]
}
