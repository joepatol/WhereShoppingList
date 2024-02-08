package models

import "gorm.io/gorm"

type Product struct {
	ID        string `gorm:"primarykey"`
	Name      string
	Store     string
	Price     float32
	Url       string
	SearchStr string `gorm:"column:searchstr"`
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
