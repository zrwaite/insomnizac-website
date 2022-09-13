package config

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
)

var CONFIG = &Config{}

func InitConfig() {
	initEnv()
	initDev()
}

func initEnv() {
	CONFIG.RedisPassword = os.Getenv("REDIS_PASSWORD")
	if CONFIG.RedisPassword == "" {
		log.Fatal("Failed to load environment variables")
	}
	CONFIG.GithubAccessToken = os.Getenv("GITHUB_ACCESS_TOKEN")
	CONFIG.SendgridAPIKey = os.Getenv("SENDGRID_API_KEY")
	CONFIG.ContactEmail = os.Getenv("CONTACT_EMAIL")
	CONFIG.FromEmail = os.Getenv("FROM_EMAIL")
}

func initDev() {
	ex, err := os.Executable()
	if err != nil {
		panic(err)
	}
	CONFIG.Directory = filepath.Dir(ex) + "/"
	if strings.Contains(ex, "var/folders") {
		CONFIG.Dev = true
		fmt.Println("dev mode enabled :)")
		CONFIG.Directory = "/Users/zacharywaite/Coding/CodeGraphs/"
	} else {
		fmt.Println("PRODUCTION MODE")
	}
}
