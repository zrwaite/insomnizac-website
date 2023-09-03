import { 
	DATABASE_USER,
	DATABASE_HOST,
	DATABASE_DB,
	DATABASE_PASSWORD,
	DATABASE_OPTIONS,
} from '$env/static/private'


export const poolOptions = {
	user: DATABASE_USER,
	host: DATABASE_HOST,
	database: DATABASE_DB,
	password: DATABASE_PASSWORD,
	port: 26257,
	options: DATABASE_OPTIONS,
	ssl: {
		rejectUnauthorized: false
	} as const
} as const
