package controller

import (
	"auth"
	"errors"
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

func GetScrapeErrorsFromDb(db *gorm.DB) ([]dto.ScrapeError, error) {
	var errors []dto.ScrapeError

	result := db.Find(&errors)
	if result.Error != nil { return nil, result.Error }

	return errors, nil
}

func GetProductsByStore(db *gorm.DB, storeName string) ([]dto.Product, error) {
	var products []dto.Product

	result := db.Where("store = ?", storeName).Find(&products)
	if result.Error != nil { return nil, result.Error }

	return products, nil
}

func GetProductsByIds(db *gorm.DB, ids []uint) ([]*models.Product, error) {
	var products []*models.Product

	result := db.Find(&products, ids) 

	if result.Error != nil {
		return nil, result.Error
	}

	return products, nil
}

func CreateShoppingList(db *gorm.DB, ownerId uint, name string, productsIds []uint) (*uint, error) {	
	products, err := GetProductsByIds(db, productsIds)

	if err != nil {
		return nil, err
	}

	user, err := auth.GetUserById(ownerId, db)
	if err != nil {
		return nil, err
	}

	var shoppingList = models.ShoppingList{
		Owner: user,
		UserID: user.ID,
		Name: name,
		Products: products,
	}

    if err := db.Create(&shoppingList).Error; err != nil {
        return nil, err
    }

	return &shoppingList.ID, nil
}

func GetAllShoppingLists(db *gorm.DB) ([]models.ShoppingList, error) {
	var shoppingLists []models.ShoppingList

	if err := db.Preload("Owner").Find(&shoppingLists).Error; err != nil {
		return nil, err
	}

	return shoppingLists, nil
}

func GetShoppingListById(db *gorm.DB, id uint64) (*dto.ShoppingList, error) {
	var shoppingList models.ShoppingList

	if err := db.Preload("Owner").Preload("Products").First(&shoppingList, id).Error; err != nil {
		return nil, errors.New("shopping list not found")
	}

	var price float32 = 0.0
	var products []*dto.Product

	for _, dbProduct := range shoppingList.Products {
		price += dbProduct.Price

		products = append(products, &dto.Product{
			ID: dbProduct.ID,
			Name: dbProduct.Name,
			Price: dbProduct.Price,
			Store: dbProduct.Store,
			Url: dbProduct.Url,
		})
	}

	dtoOwner := auth.User{
		Id: shoppingList.Owner.ID,
		Email: shoppingList.Owner.Email,
		FirstName: shoppingList.Owner.FirstName,
		LastName: shoppingList.Owner.LastName,
	}

	dtoList := dto.ShoppingList{
		ID: shoppingList.ID,
		Owner: dtoOwner,
		Name: shoppingList.Name,
		Products: products,
		TotalPrice: price,
	}

	return &dtoList, nil
}