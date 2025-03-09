package store

import "github.com/divy-sh/animus/essentias"

var urlStore = essentias.NewStringEssentia()

func GetRedirectUrl(shortId string) (string, error) {
	return urlStore.Get(shortId)
}

func SetShortUrl(shortId, url string) {
	urlStore.Set(shortId, url)
}
