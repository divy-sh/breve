package store

import "errors"

var urlStore = map[string]string{}

func GetRedirectUrl(shortId string) (string, error) {
	url, ok := urlStore[shortId]
	if !ok {
		return "", errors.New("key not found")
	}
	return url, nil
}

func SetShortUrl(shortId, url string) {
	urlStore[shortId] = url
}
