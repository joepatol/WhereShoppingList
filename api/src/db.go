package main

import (
	"context"
	"fmt"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
)

type Product struct {
	Name	string	`json:"Price"`
	Price	float32	`json:"Name"`
}

const CONN_URL = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

func connectDb() *pgxpool.Pool {
	conn, err := pgxpool.New(context.Background(), CONN_URL)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}
	return conn
}

func getProductsFromDb(pool *pgxpool.Pool) []Product {
	var query = "SELECT name, price FROM products"

	rows, err := pool.Query(context.Background(), query)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Query failed: %v\n", err)
		os.Exit(1)
	}

	var products []Product
	for rows.Next() {
		var product Product
		err := rows.Scan(&product.Name, &product.Price)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Read row failed: %v\n", err)
			os.Exit(1)
		}

		products = append(products, product)
	}
	return products
}