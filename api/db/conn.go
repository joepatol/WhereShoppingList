package db

import (
	"fmt"
	"os"
	"time"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

const CONN_URL = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

func ConnectDb() *gorm.DB {
	time.Sleep(5 * time.Second) // Wait for db startup
	connString, found := os.LookupEnv("CONN_URL")

	if !found {
		connString = CONN_URL
	}

	db, err := gorm.Open(postgres.Open(connString), &gorm.Config{})
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}
	return db
}
