package httpreq

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/zrwaite/Insomnizac/settings"
)

func GithubQuery(query string, variables string) (*http.Response, error) {
	gqlMarshalled, err := json.Marshal(QueryArgs{Query: query, Variables: variables})
	if err != nil {
		return nil, err
	}
	return AuthorizedRequest("https://api.github.com/graphql", "POST", settings.CONFIG.GithubAccessToken, strings.NewReader(string(gqlMarshalled)))
}
