package db

type Product struct {
	Name	string	`json:"name"`
	Price	float32	`json:"price"`
	Store	string	`json:"store"`
}