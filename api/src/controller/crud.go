package controller

import (
	"context"
	"db"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

func FindProductsInDb(pool *pgxpool.Pool, word string) ([]db.Product, error) {
	var query = "SELECT name, price, store FROM products WHERE searchstr LIKE '%' || LOWER($1) || '%'"

	rows, err := pool.Query(context.Background(), query, word)
	if err != nil { return nil, err }
	products, err := parseQueryResult(rows)
	if err != nil { return nil, err }
	return products, nil
}

func GetProductsFromDb(pool *pgxpool.Pool) ([]db.Product, error) {
	var query = "SELECT name, price, store FROM products"

	rows, err := pool.Query(context.Background(), query)
	if err != nil { return nil, err }
	products, err := parseQueryResult(rows)
	if err != nil { return nil, err }
	return products, nil
}

func parseQueryResult(rows pgx.Rows) ([]db.Product, error) {
	var products []db.Product
	for rows.Next() {
		var product db.Product
		err := rows.Scan(&product.Name, &product.Price, &product.Store)
		if err != nil {return nil, err}
		products = append(products, product)
	}
	return products, nil
}