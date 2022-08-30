package resolvers

import (
	"context"
	"errors"
	"fmt"

	"github.com/zrwaite/Insomnizac/graph/model"
	"github.com/zrwaite/Insomnizac/graph/services"
)

func ProjectsResolver(ctx context.Context) ([]*model.Project, error) {
	projects, status := services.GetProjects()
	if status == 200 {
		fmt.Println(projects)
		return projects, nil
	} else if status == 404 {
		return nil, errors.New("no projects found")
	} else {
		return nil, errors.New("projects query failed")
	}
}
