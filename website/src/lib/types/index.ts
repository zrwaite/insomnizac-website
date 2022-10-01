export interface GameType {
	name: string;
	slug: string;
	src: string;
}

export interface ProjectType {
	name: string
	slug: string
	description: string
	githubLink: string
	devpostLink?: string
	projectLink?: string
	image: string
	featured: boolean
}

export interface SkillType {
	name: string
	image: string 
}