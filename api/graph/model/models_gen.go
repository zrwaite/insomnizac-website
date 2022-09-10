// Code generated by github.com/99designs/gqlgen, DO NOT EDIT.

package model

type Project struct {
	ID          string   `json:"id"`
	Name        string   `json:"name"`
	Slug        string   `json:"slug"`
	Description string   `json:"description"`
	GithubName  string   `json:"githubName"`
	Languages   []string `json:"languages"`
	DevpostLink *string  `json:"devpostLink"`
	ProjectLink *string  `json:"projectLink"`
	CreatedAt   string   `json:"createdAt"`
	UpdatedAt   string   `json:"updatedAt"`
	Image       *string  `json:"image"`
}
