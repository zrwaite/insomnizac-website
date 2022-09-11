package db

import (
	"context"
	"errors"

	"github.com/go-redis/redis/v8"
	"github.com/zrwaite/Insomnizac/settings"
)

var Cache *redis.Client

func ConnectToRedis() {
	Cache = redis.NewClient(&redis.Options{
		Addr:     "localhost:6379",
		Password: settings.CONFIG.RedisPassword, // no password set
		DB:       0,                             // use default DB
	})
}

func GetCache(key string) (string, error) {
	ctx := context.Background()
	val, err := Cache.Get(ctx, key).Result()
	if err == redis.Nil {
		return "", errors.New("key does not exist")
	} else if err != nil {
		return "", err
	} else {
		return val, nil
	}
}

func SetCache(key string, value string) error {
	ctx := context.Background()
	err := Cache.Set(ctx, key, value, 0).Err()
	if err != nil {
		return err
	}
	return nil
}
