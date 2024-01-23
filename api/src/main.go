package main

import (
	"net/http"
	"controller"

	"db"
	
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type Depends struct {
	Database *gorm.DB
}

func main() {
    router := gin.Default()
	var database *gorm.DB = db.ConnectDb()
	deps := Depends{ Database: database }

    router.GET("/all_products", deps.getProducts)
	// router.GET("/find_products", deps.findProducts)
    router.Run("localhost:8080")
}

// func (deps *Depends) findProducts(ctx *gin.Context) {
// 	word := ctx.Param("word")

// 	products, err := controller.FindProductsInDb(deps.Database, word)
// 	if err != nil {
// 		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
// 	} else {
// 		ctx.IndentedJSON(http.StatusOK, products)
// 	}
// }

func (deps *Depends) getProducts(ctx *gin.Context) {
	products, err := controller.GetProductsFromDb(deps.Database)
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, products)
	}
}