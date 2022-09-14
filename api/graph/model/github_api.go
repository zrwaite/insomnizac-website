package model

type GitHubLanguage struct {
	Size int `json:"size"`
	Node struct {
		Color string `json:"color"`
		Name  string `json:"name"`
	} `json:"node"`
}

type GithubRepoResponse struct {
	Data struct {
		Repository struct {
			Description string `json:"description"`
			Languages   struct {
				TotalSize int              `json:"totalSize"`
				Edges     []GitHubLanguage `json:"edges"`
			}
		} `json:"repository"`
	} `json:"data"`
}
