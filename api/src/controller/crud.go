package controller

import (
	"models"
	"gorm.io/gorm"
)

func FindProductsInDb(db *gorm.DB, words []string) ([]models.Product, error) {
	var products []models.Product

	result := db.Where("searchstr @> ARRAY?", words).Find(&products)
	if result.Error != nil { return nil, result.Error }

	return products, nil
}

func GetProductsFromDb(db *gorm.DB) ([]models.Product, error) {
	var products []models.Product

	result := db.Find(&products)
	if result.Error != nil { return nil, result.Error }

	return products, nil
}
