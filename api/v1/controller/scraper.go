package controller

import (
	"bytes"
	"net/http"
	"dto"
	"os"
	"encoding/json"
)

const SCRAPER_URL string =  "http://localhost:7071"

func scraperUrl() string {
	connString, found := os.LookupEnv("SCRAPER_URL")

	if !found {
		connString = SCRAPER_URL
	}

	return connString
}

func GetScraperHealthCheck() (*dto.ScraperHealth, error) {
	resp, err := http.Get(scraperUrl() + "/health_check")
	if err != nil { return nil, err }
	defer resp.Body.Close()
	var dto dto.ScraperHealth;
	decode_err := json.NewDecoder(resp.Body).Decode(&dto)
	if decode_err != nil { return nil, err }
    return &dto, nil
}

func GetScraperState() (*dto.ScraperState, error) {
	resp, err := http.Get(scraperUrl() + "/status")
	if err != nil { return nil, err }
	defer resp.Body.Close()
	var dto dto.ScraperState;
	decode_err := json.NewDecoder(resp.Body).Decode(&dto)
	if decode_err != nil { return nil, err }
    return &dto, nil
}

func StartScraper() (*dto.ScraperState, error) {
	resp, err := http.Post(scraperUrl() + "/scrape_func", "application/json", bytes.NewBuffer([]byte("")))
	if err != nil { return nil, err }
	defer resp.Body.Close()
	var dto dto.ScraperState;
	decode_err := json.NewDecoder(resp.Body).Decode(&dto)
	if decode_err != nil { return nil, err }
    return &dto, nil
}