package settings

import (
	"fmt"
	"log"
	"os"

	"github.com/zrwaite/Insomnizac/models"
)

var CONFIG = &models.Config{}

func InitConfig() {
	envDev := os.Getenv("DEV")
	if envDev == "true" {
		fmt.Println("dev mode enabled :)")
		CONFIG.Dev = true
	} else if envDev == "false" {
		fmt.Println("PRODUCTION MODE")
	} else {
		log.Fatal("Failed to load environment variables")
	}
	CONFIG.GithubAccessToken = os.Getenv("GITHUB_ACCESS_TOKEN")
	CONFIG.SendgridAPIKey = os.Getenv("SENDGRID_API_KEY")
	CONFIG.ContactEmail = os.Getenv("CONTACT_EMAIL")
	CONFIG.FromEmail = os.Getenv("FROM_EMAIL")
}
