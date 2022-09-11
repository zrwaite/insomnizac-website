package queries

import (
	"fmt"
	"strings"
)

var RepositoryQuery = `
query ($name: String!) { 
	repository(name: $name, owner: "zrwaite") {
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

func GenerateRepositoriesQuery(names []string) (query string) {
	query = `query { `
	for _, name := range names {
		queryName := strings.Replace(name, "-", "", -1)
		query += fmt.Sprintf(`
		repo%s: repository(name: "%s", owner: "zrwaite") {
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
		`, queryName, name)
	}
	query += `}`
	return
}
