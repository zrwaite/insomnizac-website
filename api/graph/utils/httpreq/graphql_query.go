package httpreq

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/zrwaite/Insomnizac/config"
)

func GithubQuery(query string, variables string) (*http.Response, error) {
	gqlMarshalled, err := json.Marshal(QueryArgs{Query: query, Variables: variables})
	if err != nil {
		return nil, err
	}
	return AuthorizedRequest("https://api.github.com/graphql", "POST", config.CONFIG.GithubAccessToken, strings.NewReader(string(gqlMarshalled)))
}
