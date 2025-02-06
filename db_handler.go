package main

import "database/sql"

type UrlData struct {
	Original  string
	Shortened string
	Clicks    int
}

type ShortenerDataModel struct {
	DB *sql.DB
}

func (m *ShortenerDataModel) Latest() ([]*UrlData, error) {
	stmt := `SELECT original_url, shortened_url, clicks FROM urls`
	rows, err := m.DB.Query(stmt)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	urls := []*UrlData{}
	for rows.Next() {
		url := &UrlData{}
		err := rows.Scan(&url.Original, &url.Shortened, &url.Clicks)
		if err != nil {
			return nil, err
		}
		urls = append(urls, url)
	}

	if err = rows.Err(); err != nil {
		return nil, err
	}
	return urls, nil
}
