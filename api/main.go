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
	router.GET("/product", deps.getProductById)
	router.GET("/find_product", deps.findProducts)
	router.POST("/start_scraper", startScraper)
	router.GET("/scraper_health", getScraperHealth)
	router.GET("/scraper_state", getScraperState)
	router.GET("/scrape_errors", deps.getScrapeErrors)
	router.GET("/store", deps.getProductsByStoreName)
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
	var search_text string = query.Get("search_text")

	products, err := controller.FindProductInDb(deps.Database, search_text)
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

func (deps *Depends) getProductById(ctx *gin.Context) {
	product, err := controller.GetProductById(deps.Database, ctx.Query("id"))
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, product)
	}
}

func (deps *Depends) getScrapeErrors(ctx *gin.Context) {
	errors, err := controller.GetScrapeErrorsFromDb(deps.Database)
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, errors)
	}
}

func (deps *Depends) getProductsByStoreName(ctx *gin.Context) {
	errors, err := controller.GetProductsByStore(deps.Database, ctx.Query("store"))
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, errors)
	}
}