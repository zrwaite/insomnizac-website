package config

type Config struct {
	GithubAccessToken string
	SendgridAPIKey    string
	RedisPassword     string
	ContactEmail      string
	FromEmail         string
	Dev               bool
	Directory         string
}
