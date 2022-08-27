package settings

import (
	"fmt"
	"log"
	"os"
)

var DEV = true

func MatchDev() {
	envDev := os.Getenv("DEV")
	if envDev == "true" {
		fmt.Println("dev mode enabled :)")
	} else if envDev == "false" {
		fmt.Println("PRODUCTION MODE")
		DEV = false
	} else {
		log.Fatal("Failed to load environment variables")
	}
}
