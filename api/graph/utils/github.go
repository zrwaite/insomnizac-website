package utils

import "strings"

func GetProjectNames(githubName string) (username, projectName string) {
	if !strings.Contains(githubName, "/") {
		username = "zrwaite"
		projectName = githubName
	} else {
		names := strings.Split(githubName, "/")
		username = names[0]
		projectName = names[1]
	}
	return
}

func GetQueryName(githubName string) string {
	_, projectName := GetProjectNames(githubName)
	return "repo" + strings.Replace(projectName, "-", "", -1)
}
