package models

type Product struct {
	ID			string `gorm:"primarykey"`
	Name		string
	Store 		string
	Price 		float32
	Url			string
	SearchStr 	string `gorm:"column:searchstr"`
}

type ScrapeError struct {
	ID			string `gorm:"primarykey"`
	Scraper 	string
	Message		string
}