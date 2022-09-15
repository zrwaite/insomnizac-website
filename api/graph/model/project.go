package model

import (
	"strings"
)

func (project *Project) ParseProject() {
	var username string
	if !strings.Contains(project.GithubName, "/") {
		username = "zrwaite/"
	}
	project.GithubLink = "https://github.com/" + username + project.GithubName
}
