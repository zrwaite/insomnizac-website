import type { ProjectType } from '$lib/types'
import { GITHUB_ACCESS_TOKEN } from '$env/static/private'

const getProjectNames = (githubName: string): { username: string, projectName: string } => {
	if (!githubName.includes('/')) {
		return { username: 'zrwaite', projectName: githubName }
	} else {
		const [username, projectName] = githubName.split('/')
		return { username, projectName }
	}
}

const getQueryName = (githubName: string): string => {
	const { projectName } = getProjectNames(githubName)
	return 'repo' + projectName.replace('-', '')
}


export const generateRepositoriesQuery = (githubNames: string[]): string => {
	let query = 'query { '
	for (const githubName of githubNames) {
		const { username, projectName } = getProjectNames(githubName)
		const queryName = getQueryName(githubName)
		query += `
		${queryName}: repository(name: "${projectName}", owner: "${username}") {
			description
			languages(first: 10, orderBy: {
				direction: DESC,
				field: SIZE
			}) 
			{
				totalSize
				edges {
					size
					node {
						color
						name
					}
				}
			}
		}
	`
	}
	query += '}'
	return query
}

export const getRepositoriesData = async (projects: ProjectType[]): Promise<ProjectType[]> => {
	const repoGithubNames = projects.map((project) => project.github_name)
	const repoQuery = generateRepositoriesQuery(repoGithubNames)
	const resp = await githubQuery(repoQuery, {})
	if (resp.status !== 200) {
		throw new Error('github API returned non-200 status code')
	}
	const body = await resp.json()
	const data = body.data
	for (const project of projects) {
		const key = getQueryName(project.github_name)
		const repo = data[key]
		project.description = repo.description
		// project.languages = repo.languages.edges.map((edge) => edge.node)
	}
	return projects
}

const githubQuery = async (query: string, variables: { [key: string]: string }): Promise<Response> => {
	const variablesJson = JSON.stringify(variables)
	const body = JSON.stringify({ query, variables: variablesJson })
	const resp = await authorizedRequest('https://api.github.com/graphql', 'POST', GITHUB_ACCESS_TOKEN, body)
	return resp
}

const authorizedRequest = async (url: string, method: string, token: string, body: string): Promise<Response> => {
	const bearerToken = 'Bearer ' + token
	const resp = await fetch(url, {
		method,
		headers: {
			Authorization: bearerToken,
			'Content-Type': 'application/json'
		},
		body
	})
	return resp
}


// func GetRepositoriesData(projects []*model.Project) error {
// 	repoGithubNames := []string{}
// 	for _, project := range projects {
// 		repoGithubNames = append(repoGithubNames, project.GithubName)
// 	}
// 	repoQuery := queries.GenerateRepositoriesQuery(repoGithubNames)
// 	resp, err := httpreq.GithubQuery(repoQuery, emptyMap)
// 	if err != nil {
// 		return err
// 	}
// 	if resp.StatusCode != 200 {
// 		return errors.New("github API returned non-200 status code")
// 	}
// 	var body map[string]interface{}
// 	err = json.NewDecoder(resp.Body).Decode(&body)
// 	if err != nil {
// 		return err
// 	}
// 	data := body["data"].(map[string]interface{})
// 	for _, project := range projects {
// 		key := utils.GetQueryName(project.GithubName)
// 		repo := data[key].(map[string]interface{})
// 		project.Description = repo["description"].(string)
// 	}
// 	return nil
// }
