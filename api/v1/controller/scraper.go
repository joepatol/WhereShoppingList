package controller

import (
	"bytes"
	"net/http"
	"dto"
	"encoding/json"
)

const START_SCRAPE_URL string = "http://localhost:7071/scrape_func"
const SCRAPE_STATE_URL string = "http://localhost:7071/status"
const SCRAPE_HEALTH_URL string = "http://localhost:7071/health_check"

func GetScraperHealthCheck() (*dto.ScraperHealth, error) {
	resp, err := http.Get(SCRAPE_HEALTH_URL)
	if err != nil { return nil, err }
	defer resp.Body.Close()
	var dto dto.ScraperHealth;
	decode_err := json.NewDecoder(resp.Body).Decode(&dto)
	if decode_err != nil { return nil, err }
    return &dto, nil
}

func GetScraperState() (*dto.ScraperState, error) {
	resp, err := http.Get(SCRAPE_STATE_URL)
	if err != nil { return nil, err }
	defer resp.Body.Close()
	var dto dto.ScraperState;
	decode_err := json.NewDecoder(resp.Body).Decode(&dto)
	if decode_err != nil { return nil, err }
    return &dto, nil
}

func StartScraper() (*dto.ScraperState, error) {
	resp, err := http.Post(START_SCRAPE_URL, "application/json", bytes.NewBuffer([]byte("")))
	if err != nil { return nil, err }
	defer resp.Body.Close()
	var dto dto.ScraperState;
	decode_err := json.NewDecoder(resp.Body).Decode(&dto)
	if decode_err != nil { return nil, err }
    return &dto, nil
}