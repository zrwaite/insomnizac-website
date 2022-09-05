package services

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"strings"

	"github.com/zrwaite/Insomnizac/database"
	"github.com/zrwaite/Insomnizac/graph/model"
	"github.com/zrwaite/Insomnizac/graph/services/queries"
	httpreq "github.com/zrwaite/Insomnizac/graph/utils/http"
)

func GetProjectArgs(project *model.Project) []interface{} {
	return []interface{}{&project.ID, &project.Name, &project.Slug, &project.GithubName, &project.DevpostLink, &project.ProjectLink, &project.CreatedAt, &project.UpdatedAt}
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
	err = GetRepositoriesData(projects)
	if err != nil {
		log.Fatal(err)
	}
	return projects, 200
}

func GetRepositoriesData(projects []*model.Project) error {
	repoNames := []string{}
	for _, project := range projects {
		repoNames = append(repoNames, project.GithubName)
	}
	repoQuery := queries.GenereateRepositoriesQuery(repoNames)
	resp, err := httpreq.GithubQuery(repoQuery, "")
	if err != nil {
		return err
	}
	if resp.StatusCode != 200 {
		return errors.New("github API returned non-200 status code")
	}
	var body map[string]interface{}
	err = json.NewDecoder(resp.Body).Decode(&body)
	if err != nil {
		return err
	}
	data := body["data"].(map[string]interface{})
	for _, project := range projects {
		key := "repo" + strings.Replace(project.GithubName, "-", "", -1)
		repo := data[key].(map[string]interface{})
		project.Description = repo["description"].(string)
	}
	return nil
}
