package main

import (
	"context"
	"fmt"
	"os"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

const CONN_URL = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

type Product struct {
	Name	string	`json:"name"`
	Price	float32	`json:"price"`
	Store	string	`json:"store"`
}

func connectDb() *pgxpool.Pool {
	conn, err := pgxpool.New(context.Background(), CONN_URL)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}
	return conn
}

func findProductsInDb(pool *pgxpool.Pool, word string) []Product {
	var query = "SELECT name, price, store FROM products WHERE searchstr LIKE '%' || LOWER($1) || '%'"

	rows, err := pool.Query(context.Background(), query, word)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Query failed: %v\n", err)
		os.Exit(1)
	}
	return parseQueryResult(rows)
}

func getProductsFromDb(pool *pgxpool.Pool) []Product {
	var query = "SELECT name, price, store FROM products"

	rows, err := pool.Query(context.Background(), query)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Query failed: %v\n", err)
		os.Exit(1)
	}
	return parseQueryResult(rows)
}

func parseQueryResult(rows pgx.Rows) []Product {
	var products []Product
	for rows.Next() {
		var product Product
		err := rows.Scan(&product.Name, &product.Price, &product.Store)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Read row failed: %v\n", err)
			os.Exit(1)
		}

		products = append(products, product)
	}
	return products
}