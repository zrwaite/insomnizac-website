package main

import (
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/joho/godotenv"
	"github.com/zrwaite/Insomnizac/database"
	"github.com/zrwaite/Insomnizac/graph"
	"github.com/zrwaite/Insomnizac/graph/generated"
	"github.com/zrwaite/Insomnizac/graph/services/queries"
	"github.com/zrwaite/Insomnizac/settings"
)

const defaultPort = "8011"

func main() {
	godotenv.Load(".env")
	settings.InitConfig()
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}
	database.ConnectToDB()

	query := queries.GenereateRepositoriesQuery([]string{"Insomnizac", "zrwaite", "HomeNode"})
	fmt.Println(query)

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &graph.Resolver{}}))

	http.Handle("/graphql_playground", playground.Handler("GraphQL playground", "/graphql"))
	http.Handle("/graphql", srv)

	log.Printf("connect to http://localhost:%s/graphql_playground for GraphQL playground", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
