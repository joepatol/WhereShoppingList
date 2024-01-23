package main

import (
	"net/http"
	"db"
	"controller"
	
	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgxpool"
)

type Depends struct {
	ConnPool *pgxpool.Pool
}

func main() {
    router := gin.Default()
	var pool *pgxpool.Pool = db.ConnectDb()
	defer pool.Close()
	deps := Depends{ ConnPool: pool }

    router.GET("/all_products", deps.getProducts)
	router.GET("/find_products", deps.findProducts)
    router.Run("localhost:8080")
}

func (deps *Depends) findProducts(ctx *gin.Context) {
	word := ctx.Param("word")

	products, err := controller.FindProductsInDb(deps.ConnPool, word)
	if err != nil {
		ctx.Status(http.StatusInternalServerError)
	} else {
		ctx.IndentedJSON(http.StatusOK, products)
	}
}

func (deps *Depends) getProducts(ctx *gin.Context) {
	products, err := controller.GetProductsFromDb(deps.ConnPool)
	if err != nil {
		ctx.Status(http.StatusInternalServerError)
	} else {
		ctx.IndentedJSON(http.StatusOK, products)
	}
}