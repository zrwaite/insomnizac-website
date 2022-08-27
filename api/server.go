package main

import (
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/zrwaite/Insomnizac/graph"
	"github.com/zrwaite/Insomnizac/graph/generated"
)

const defaultPort = "8011"

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &graph.Resolver{}}))

	http.Handle("/graphql_playground", playground.Handler("GraphQL playground", "/graphql"))
	http.Handle("/graphql", srv)

	log.Printf("connect to http://localhost:%s/graphql_playground for GraphQL playground", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
