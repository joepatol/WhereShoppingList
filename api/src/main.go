package main

import (
	"net/http"
	
	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgxpool"
)

func main() {
    router := gin.Default()
    router.GET("/all_products", getProducts)
	router.GET("/find_products", findProducts)
    router.Run("localhost:8080")
}

func findProducts(ctx *gin.Context) {
	var pool *pgxpool.Pool = connectDb()
	defer pool.Close()

	word := ctx.Param("word")

	var products []Product = findProductsInDb(pool, word)
	ctx.IndentedJSON(http.StatusOK, products)
}

func getProducts(ctx *gin.Context) {
	var pool *pgxpool.Pool = connectDb()
	defer pool.Close()
	var products []Product = getProductsFromDb(pool)
    ctx.IndentedJSON(http.StatusOK, products)
}