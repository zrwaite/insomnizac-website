package queries

import (
	"fmt"

	"github.com/zrwaite/Insomnizac/utils"
)

var RepositoryQuery = `
query ($name: String!, $owner: String!) { 
	repository(name: $name, owner: $owner) {
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
}
`

func GenerateRepositoriesQuery(githubNames []string) (query string) {
	query = `query { `
	for _, githubName := range githubNames {
		username, projectName := utils.GetProjectNames(githubName)
		queryName := utils.GetQueryName(githubName)
		query += fmt.Sprintf(`
		%s: repository(name: "%s", owner: "%s") {
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
		`, queryName, projectName, username)
	}
	query += `}`
	return
}
