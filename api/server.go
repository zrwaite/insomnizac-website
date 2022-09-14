package main

import (
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/joho/godotenv"
	"github.com/zrwaite/Insomnizac/config"
	"github.com/zrwaite/Insomnizac/db"
	"github.com/zrwaite/Insomnizac/graph"
	"github.com/zrwaite/Insomnizac/graph/generated"
)

const defaultPort = "8011"

func serverMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		next.ServeHTTP(w, r)
	})
}

func main() {
	godotenv.Load(".env")
	config.InitConfig()
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	db.ConnectToDB()
	db.ConnectToRedis()

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &graph.Resolver{}}))

	http.Handle("/graphql_playground", playground.Handler("GraphQL playground", "/graphql"))
	http.Handle("/graphql", serverMiddleware(srv))

	log.Printf("connect to http://localhost:%s/graphql_playground for GraphQL playground", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
