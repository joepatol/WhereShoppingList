package db

import (
	"fmt"
	"os"
	"gorm.io/gorm"
	"gorm.io/driver/postgres"
)

const CONN_URL = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

func ConnectDb() *gorm.DB {
	db, err := gorm.Open(postgres.Open(CONN_URL), &gorm.Config{})
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}
	return db
}
