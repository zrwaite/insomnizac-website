package models

type Config struct {
	GithubAccessToken string
	SendgridAPIKey    string
	RedisPassword     string
	ContactEmail      string
	FromEmail         string
	Dev               bool
}
