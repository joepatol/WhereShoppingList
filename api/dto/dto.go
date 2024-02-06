package dto

type Product struct {
	ID		string	`json:"id"`
	Name	string	`json:"name"`
	Price	float32	`json:"price"`
	Store	string	`json:"store"`
	Url		string	`json:"url"`
}

type ScrapeError struct {
	Scraper		string 	`json:"scraper"`
	Message		string	`json:"message"`
}

type ScraperState struct {
	Status	string	`json:"status"`
}

type ScraperHealth struct {
	State string `json:"state"`
}