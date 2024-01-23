package main

import (
	"net/http"
	"log"
	"controller"
	"db"
	
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type Depends struct {
	Database *gorm.DB
	Logger *log.Logger
}

func main() {
    router := gin.Default()
	var database *gorm.DB = db.ConnectDb()
	deps := Depends{ 
		Database: database,
		Logger: log.Default(),
	}

    router.GET("/all_products", deps.getProducts)
	router.GET("/find_products", deps.findProducts)
    router.Run("localhost:8080")
}

func (deps *Depends) findProducts(ctx *gin.Context) {
	query := ctx.Request.URL.Query()
	var words []string = query["words"]

	products, err := controller.FindProductsInDb(deps.Database, words)
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, products)
	}
}

func (deps *Depends) getProducts(ctx *gin.Context) {
	products, err := controller.GetProductsFromDb(deps.Database)
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, products)
	}
}