package models

import "gorm.io/gorm"

type Product struct {
	ID        string `gorm:"primarykey"`
	Name      string
	Store     string
	Price     float32
	Url       string
}

type ScrapeError struct {
	ID      string `gorm:"primarykey"`
	Scraper string
	Message string
}

type User struct {
	gorm.Model
	FirstName string	`gorm:"size:255;not null;unique" json:"first_name"`
	LastName  string	`gorm:"size:255;not null;unique" json:"last_name"`
	Email     string	`gorm:"size:255;not null;unique" json:"email"`
	Password  string	`gorm:"size:255;not null;unique" json:"password"`
}

type ShoppingList struct {
	gorm.Model
	Owner		*User		`json:"owner"`
	Name		string		`gorm:"size:255;not null" json:"name"`
	Products	[]*Product	`gorm:"many2many:shopping_list_products" json:"products"`
}

type ShoppingListProduct struct {
    gorm.Model
    ShoppingListID uint
    ProductID      uint
}