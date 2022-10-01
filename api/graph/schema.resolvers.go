package graph

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"

	"github.com/zrwaite/Insomnizac/graph/generated"
	"github.com/zrwaite/Insomnizac/graph/model"
	"github.com/zrwaite/Insomnizac/graph/resolvers"
)

func (r *mutationResolver) Contact(ctx context.Context, message string) (bool, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *projectResolver) Skills(ctx context.Context, obj *model.Project) ([]*model.Skill, error) {
	return resolvers.ProjectSkillsResolver(obj)
}

func (r *queryResolver) Projects(ctx context.Context) ([]*model.Project, error) {
	return resolvers.ProjectsResolver()
}

func (r *queryResolver) Project(ctx context.Context, slug string) (*model.Project, error) {
	return resolvers.ProjectResolver(slug)
}

func (r *queryResolver) Skills(ctx context.Context) ([]*model.Skill, error) {
	return resolvers.SkillsResolver()
}

// Mutation returns generated.MutationResolver implementation.
func (r *Resolver) Mutation() generated.MutationResolver { return &mutationResolver{r} }

// Project returns generated.ProjectResolver implementation.
func (r *Resolver) Project() generated.ProjectResolver { return &projectResolver{r} }

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type mutationResolver struct{ *Resolver }
type projectResolver struct{ *Resolver }
type queryResolver struct{ *Resolver }
