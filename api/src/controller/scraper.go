package controller

import (
	"dto"
)

const START_SCRAPE_URL string = "http://scrape-func:7071/scrap_func"
const SCRAPE_STATE_URL string = "http://scrape-func:7071/state"
const SCRAPE_HEALTH_URL string = "http://scrape-func:7071/health_check"

func GetScraperState() dto.ScraperState {

}

func StartScraper() dto.ScraperState {

}