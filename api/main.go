package main

import (
	"net/http"
	"log"
	"controller"
	"db"
	
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)


// Dependency injection
type Depends struct {
	Database *gorm.DB
	Logger *log.Logger
}

// Helpers
func sendResponseOrError (ctx *gin.Context, obj any, err error) {
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, obj)
	}
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

// Route handlers
func getScraperHealth(ctx *gin.Context) {
	json, err := controller.GetScraperHealthCheck()
	sendResponseOrError(ctx, json, err)
}

func startScraper(ctx *gin.Context) {
	json, err := controller.StartScraper()
	sendResponseOrError(ctx, json, err)
}

func getScraperState(ctx *gin.Context) {
	json, err := controller.GetScraperState()
	sendResponseOrError(ctx, json, err)
}

func (deps *Depends) findProducts(ctx *gin.Context) {
	query := ctx.Request.URL.Query()
	var search_text string = query.Get("search_text")

	products, err := controller.FindProductInDb(deps.Database, search_text)
	sendResponseOrError(ctx, products, err)
}

func (deps *Depends) getProducts(ctx *gin.Context) {
	products, err := controller.GetProductsFromDb(deps.Database)
	sendResponseOrError(ctx, products, err)
}

func (deps *Depends) getProductById(ctx *gin.Context) {
	product, err := controller.GetProductById(deps.Database, ctx.Query("id"))
	sendResponseOrError(ctx, product, err)
}

func (deps *Depends) getScrapeErrors(ctx *gin.Context) {
	errors, err := controller.GetScrapeErrorsFromDb(deps.Database)
	sendResponseOrError(ctx, errors, err)
}

func (deps *Depends) getProductsByStoreName(ctx *gin.Context) {
	products, err := controller.GetProductsByStore(deps.Database, ctx.Query("store"))
	sendResponseOrError(ctx, products, err)
}