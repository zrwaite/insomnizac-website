package database

import "go.mongodb.org/mongo-driver/bson"

var bsonStringArray = bson.M{
	"bsonType":    "array",
	"uniqueItems": true,
	"items":       bson.M{"bsonType": "string"},
}

var bsonString = bson.M{"bsonType": "string"}

var ProjectsValidator = bson.M{
	"$jsonSchema": bson.M{
		"bsonType": "object",
		"required": []string{"id", "name"},
		"properties": bson.M{
			"id":   bsonString,
			"name": bsonString,
		},
	},
}
