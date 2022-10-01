package resolvers

import (
	"errors"

	"github.com/zrwaite/Insomnizac/graph/model"
	"github.com/zrwaite/Insomnizac/graph/services"
)

func SkillsResolver() ([]*model.Skill, error) {
	projects, status := services.GetSkills()
	if status == 200 {
		return projects, nil
	} else if status == 404 {
		return nil, errors.New("no projects found")
	} else {
		return nil, errors.New("projects query failed")
	}
}
