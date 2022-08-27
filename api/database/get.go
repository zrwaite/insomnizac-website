package database

import (
	"context"
	"fmt"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

func Get(collectionName string, filter primitive.D, parseInto interface{}) (status int) {
	cursor := mongoDatabase.Collection(collectionName).FindOne(context.TODO(), filter)
	if err := cursor.Decode(parseInto); err != nil {
		if err.Error() == "mongo: no documents in result" {
			status = 404
		} else {
			fmt.Println("Failed to get from table " + collectionName + " ; " + err.Error())
			status = 400
		}
		return
	} else {
		status = 200
	}
	return
}
