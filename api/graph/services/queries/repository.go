package queries

import "fmt"

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

func GenereateRepositoriesQuery(names []string) (query string) {
	query = `query { `
	for _, name := range names {
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
		`, name, name)
	}
	query += `}`
	return
}
