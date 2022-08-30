package services

import (
	"github.com/zrwaite/Insomnizac/database"
	"github.com/zrwaite/Insomnizac/graph/model"
)

func GetProjects() (projects []*model.Project, status int) {
	rows, err := database.DB.Query("SELECT * FROM projects")
	defer rows.Close()
	if err != nil {
		return nil, 400
	}
	for rows.Next() {
		var project *model.Project
		rows.Scan(&project)
		projects = append(projects, project)
	}
	return projects, 200
}
