CREATE TABLE IF NOT EXISTS "urls" (
	original_url TEXT PRIMARY KEY NOT NULL,
	shortened_url TEXT NOT NULL,
	clicks INTEGER DEFAULT 0,
	created DATETIME DEFAULT CURRENT_TIMESTAMP,
	updated DATETIME DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT uniq_original_url UNIQUE (original_url)
);

CREATE index IF NOT EXISTS idx_shortened ON urls (shortened_url);

INSERT INTO urls (original_url, shortened_url, clicks) 
VALUES
("https://osnews.com", "shoRtkl9187ds", 347),
("https://stackoverflow.com/questions/tagged/go", "sh0Rtkl9187es", 2809);
