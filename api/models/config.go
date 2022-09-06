package models

type Config struct {
	GithubAccessToken string
	SendgridAPIKey    string
	ContactEmail      string
	FromEmail         string
	Dev               bool
}
