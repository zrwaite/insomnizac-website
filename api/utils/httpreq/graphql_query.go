package httpreq

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/zrwaite/Insomnizac/config"
)

func GithubQuery(query string, variables map[string]string) (*http.Response, error) {
	variablesJson, err := json.Marshal(variables)
	if err != nil {
		return nil, err
	}
	variablesString := string(variablesJson)
	gqlMarshalled, err := json.Marshal(QueryArgs{Query: query, Variables: variablesString})
	if err != nil {
		return nil, err
	}
	return AuthorizedRequest("https://api.github.com/graphql", "POST", config.CONFIG.GithubAccessToken, strings.NewReader(string(gqlMarshalled)))
}
