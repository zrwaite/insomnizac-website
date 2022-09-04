package models

type GraphQLQuery struct {
	Query     string `json:"query"`
	Variables string `json:"variables"`
}
