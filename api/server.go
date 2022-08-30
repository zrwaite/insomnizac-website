package main

import (
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/joho/godotenv"
	"github.com/zrwaite/Insomnizac/database"
	"github.com/zrwaite/Insomnizac/graph"
	"github.com/zrwaite/Insomnizac/graph/generated"
)

const defaultPort = "8011"

func main() {
	godotenv.Load(".env")
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}
	database.ConnectToDB()

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &graph.Resolver{}}))

	http.Handle("/graphql_playground", playground.Handler("GraphQL playground", "/graphql"))
	http.Handle("/graphql", srv)

	log.Printf("connect to http://localhost:%s/graphql_playground for GraphQL playground", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
