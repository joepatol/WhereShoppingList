package main

import (
	"net/http"
	
	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgxpool"
)

func main() {
    router := gin.Default()
    router.GET("/all_products", getProducts)
    router.Run("localhost:8080")
}

func getProducts(c *gin.Context) {
	var pool *pgxpool.Pool = connectDb()
	defer pool.Close()
	var products []Product = getProductsFromDb(pool)
    c.IndentedJSON(http.StatusOK, products)
}