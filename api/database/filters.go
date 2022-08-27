package database

import (
	"fmt"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func CreateUsernameFilter(username string) bson.D {
	return bson.D{{
		Key: "username",
		Value: bson.D{{
			Key:   "$eq",
			Value: username,
		}},
	}}
}

func CreateIdFilter(id string) (filter bson.D, success bool) {
	objectId, err := primitive.ObjectIDFromHex(id)
	if err != nil {
		fmt.Println("Failed to parse id " + id + " ; " + err.Error())
		return nil, false
	}
	return bson.D{{
		Key: "_id",
		Value: bson.D{{
			Key:   "$eq",
			Value: objectId,
		}},
	}}, true
}

func CreatePartialUsernameFilter(partialUsername string) bson.D {
	return bson.D{{
		Key: "username",
		Value: bson.M{"$regex": primitive.Regex{
			Pattern: partialUsername,
			Options: "i",
		}},
	}}
}
