package dto

import "models"

type Product struct {
	ID    string  `json:"id"`
	Name  string  `json:"name"`
	Price float32 `json:"price"`
	Store string  `json:"store"`
	Url   string  `json:"url"`
}

type ScrapeError struct {
	Scraper string `json:"scraper"`
	Message string `json:"message"`
}

type ScraperState struct {
	Status string `json:"status"`
}

type ScraperHealth struct {
	State string `json:"state"`
}

type CreateShoppingListInput struct {
	OwnerId 	uint	`json:"owner_id"`
	Name 		string	`json:"name"`
	ProductIds 	[]uint	`json:"product_ids"`
}

type ShoppingList struct {
	ID		   uint		   	`json:"id"`
	Owner      models.User 	`json:"owner"`
	Name       string      	`json:"name"`
	Products   []*Product   `json:"products"`
	TotalPrice float32     `json:"total_price"`
}