package models

import "gorm.io/gorm"

type Product struct {
	ID        		uint `gorm:"primarykey"`
	Name      		string
	Store     		string
	Price     		float32
	Url       		string
	ShoppingLists	[]*ShoppingList `gorm:"many2many:products_shoppinglists"`
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
	Owner		*User		`gorm:"foreignKey:UserID" json:"owner"`
	UserID		uint
	Name		string		`gorm:"size:255;not null" json:"name"`
	Products	[]*Product	`gorm:"many2many:products_shoppinglists" json:"products"`
}
