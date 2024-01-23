package controller

import (
	"models"
	"gorm.io/gorm"
)

// func FindProductsInDb(db *gorm.DB, word string) ([]models.Product, error) {
// 	var query = "SELECT name, price, store FROM products WHERE searchstr @> $1"

// 	words := []string{ word }

// 	rows, err := db.Query(context.Background(), query, words)
// 	if err != nil { return nil, err }
// 	products, err := parseQueryResult(rows)
// 	if err != nil { return nil, err }
// 	return products, nil
// }

func GetProductsFromDb(db *gorm.DB) ([]models.Product, error) {
	var products []models.Product

	result := db.Find(&products)
	if result.Error != nil { return nil, result.Error }

	return products, nil
}
