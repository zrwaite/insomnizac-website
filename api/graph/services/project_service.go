package services

import (
	"fmt"

	"github.com/zrwaite/Insomnizac/database"
	"github.com/zrwaite/Insomnizac/graph/model"
)

func GetProjectArgs(project *model.Project) []interface{} {
	return []interface{}{&project.ID, &project.Name, &project.Slug, &project.Description, &project.GithubName, &project.DevpostLink, &project.ProjectLink, &project.CreatedAt, &project.UpdatedAt}
}

func GetGithubProject(project *model.Project) {

}

func GetProjects() (projects []*model.Project, status int) {
	rows, err := database.DB.Query("SELECT * FROM projects")
	if err != nil {
		return nil, 400
	}
	defer rows.Close()
	for rows.Next() {
		project := new(model.Project)
		err = rows.Scan(GetProjectArgs(project)...)
		if err != nil {
			fmt.Println(err)
			return nil, 400
		}
		projects = append(projects, project)
	}
	return projects, 200
}
