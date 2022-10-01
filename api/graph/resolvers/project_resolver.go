package resolvers

import (
	"errors"

	"github.com/zrwaite/Insomnizac/graph/model"
	"github.com/zrwaite/Insomnizac/graph/services"
)

func ProjectsResolver() ([]*model.Project, error) {
	projects, status := services.GetProjects()
	if status == 200 {
		return projects, nil
	} else if status == 404 {
		return nil, errors.New("no projects found")
	} else {
		return nil, errors.New("projects query failed")
	}
}

func ProjectResolver(slug string) (*model.Project, error) {
	project, status := services.GetProject(slug)
	if status == 200 {
		return project, nil
	} else if status == 404 {
		return nil, errors.New("project not found")
	} else {
		return nil, errors.New("project query failed")
	}
}

func ProjectSkillsResolver(obj *model.Project) ([]*model.Skill, error) {
	return services.GetProjectSkills(obj)
}
