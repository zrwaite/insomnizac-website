package database

import (
	"context"
	"log"

	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo/options"
)

func InitializeDatabase() {
	InitializeCollection("projects", ProjectsValidator)
}

func InitializeCollection(name string, validator primitive.M) {
	mongoDatabase.Collection(name).Drop(context.TODO())
	options := new(options.CreateCollectionOptions)
	options.Validator = validator
	error := mongoDatabase.CreateCollection(context.TODO(), name, options)
	if error != nil {
		log.Fatal("Failed to create " + name + " collection; " + error.Error())
	}
}
