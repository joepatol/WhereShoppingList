package models

type Product struct {
	ID			string `gorm:"primarykey"`
	Name		string
	Store 		string
	Price 		float32
	SearchStr 	string `gorm:"column:searchstr"`
}