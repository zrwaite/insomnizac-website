package httpreq

import (
	"io"
	"log"
	"net/http"
)

func AuthorizedGet(url string, bearerToken string) (*http.Response, error) {
	return AuthorizedRequest(url, "GET", bearerToken, nil)
}

func AuthorizedRequest(url string, method string, token string, body io.Reader) (*http.Response, error) {
	bearerToken := "Bearer " + token
	req, err := http.NewRequest(method, url, body)
	if err != nil {
		log.Fatal("NewRequest: ", err)
	}
	req.Header.Add("Authorization", bearerToken)
	client := &http.Client{}
	return client.Do(req)
}
