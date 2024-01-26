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
	deps := Depends{ 
		Database: db.ConnectDb(),
		Logger: log.Default(),
	}

    router.GET("/all_products", deps.getProducts)
	router.GET("/find_products", deps.findProducts)
	router.POST("/start_scraper", startScraper)
	router.GET("/scraper_health", getScraperHealth)
	router.GET("/scraper_state", getScraperState)
    router.Run("localhost:8080")
}

func getScraperHealth(ctx *gin.Context) {
	json, err := controller.GetScraperHealthCheck()
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, json)
	}
}

func startScraper(ctx *gin.Context) {
	json, err := controller.StartScraper()
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, json)
	}
}

func getScraperState(ctx *gin.Context) {
	json, err := controller.GetScraperState()
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, json)
	}	
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