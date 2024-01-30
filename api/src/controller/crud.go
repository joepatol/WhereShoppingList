package controller

import (
	"dto"
	"models"
	"gorm.io/gorm"
)

func FindProductInDb(db *gorm.DB, search_text string) ([]dto.Product, error) {
	query := `
		SELECT *, ts_rank(to_tsvector('dutch', searchstr), plainto_tsquery('dutch', ?)) AS rank FROM products
		WHERE to_tsvector('dutch', searchstr) @@ plainto_tsquery('dutch', ?)
		ORDER BY rank DESC
		LIMIT 10
	`

	var products []dto.Product

	result := db.Raw(query, search_text, search_text).Find(&products)
	if result.Error != nil { return nil, result.Error }

	return products, nil
}

func GetProductById(db *gorm.DB, id string) (*dto.Product, error) {
	var product dto.Product

	result := db.Where("id = ?", id).First(&product)
	if result.Error != nil { return nil, result.Error }

	return &product, nil
}

func GetProductsFromDb(db *gorm.DB) ([]models.Product, error) {
	var products []models.Product

	result := db.Find(&products)
	if result.Error != nil { return nil, result.Error }

	return products, nil
}
