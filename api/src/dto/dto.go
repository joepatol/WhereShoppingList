package dto

type Product struct {
	Name	string	`json:"name"`
	Price	float32	`json:"price"`
	Store	string	`json:"store"`
}

type ScraperState struct {
	Status	string	`json:"status"`
}