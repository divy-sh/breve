package handlers

import (
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"strings"
	"text/template"

	"github.com/divy-sh/breve/store"
)

func Handle() {
	http.HandleFunc("/", serveIndex)
	http.HandleFunc("/shorten", shortenURL)
	http.HandleFunc("/r/", redirectURL)
	log.Println("Server running on port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}

// Serve the index.html page
func serveIndex(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/" {
		http.NotFound(w, r)
		return
	}
	tmpl := template.Must(template.ParseFiles("views/index.html"))
	tmpl.Execute(w, nil)
}

// Handle URL shortening
func shortenURL(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Invalid request", http.StatusMethodNotAllowed)
		return
	}
	r.ParseForm()
	longURL := r.Form.Get("url")
	if longURL == "" {
		http.Error(w, "Missing URL", http.StatusBadRequest)
		return
	}
	shortID := generateShortID()
	store.SetShortUrl(shortID, longURL)
	originAddr := r.Header.Get("Origin")
	fmt.Fprintf(w, `<p>Shortened URL: <a href="%s/r/%s" target="_blank">%s/r/%s</a></p>`, originAddr, shortID, originAddr, shortID)
}

// Handle redirection
func redirectURL(w http.ResponseWriter, r *http.Request) {
	shortID := strings.TrimPrefix(r.URL.Path, "/r/")
	longURL, err := store.GetRedirectUrl(shortID)
	if err != nil {
		http.NotFound(w, r)
	} else {
		http.Redirect(w, r, longURL, http.StatusFound)
	}
}

// Generate a short ID
func generateShortID() string {
	const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	shortID := make([]byte, 6)
	for i := range shortID {
		shortID[i] = charset[rand.Intn(len(charset))]
	}
	return string(shortID)
}
