package v1

import (
	"auth"
	"controller"
	"core"
	"dto"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

type Depends core.Depends

func Add(base gin.IRouter, cdeps core.Depends) {
    router := base.Group("v1")
	router.Use(auth.JwtAuthMiddleware())
	deps := Depends(cdeps)

    router.GET("/all_products", deps.getProducts)
	router.GET("/product", deps.getProductById)
	router.GET("/find_product", deps.findProducts)
	router.POST("/start_scraper", startScraper)
	router.GET("/scraper_health", getScraperHealth)
	router.GET("/scraper_state", getScraperState)
	router.GET("/scrape_errors", deps.getScrapeErrors)
	router.GET("/store", deps.getProductsByStoreName)
	router.POST("/shopping_list", deps.createShoppingList)
	router.GET("/shopping_list", deps.getShoppingListById)
}

func (deps *Depends) getShoppingListById(ctx *gin.Context) {
	id, err := strconv.ParseUint(ctx.Query("id"), 10, 32)
	if err != nil {
		deps.Logger.Println("Not a valid id value: ", err.Error())
		ctx.AbortWithStatus(http.StatusBadRequest)
	}

	shoppingList, err := controller.GetShoppingListById(deps.Database, id)
	core.SendResponseOrError(ctx, shoppingList, err)
}

func (deps *Depends) createShoppingList(ctx *gin.Context) {
	var data dto.CreateShoppingListInput

	if err := ctx.ShouldBindJSON(&data); err != nil {
		deps.Logger.Println("Bad request data: ", err.Error())
		ctx.AbortWithStatus(http.StatusBadRequest)
	}

	shoppingListId, err := controller.CreateShoppingList(
		deps.Database,
		data.OwnerId,
		data.Name,
		data.ProductIds,
	)

	if err != nil { 
		deps.Logger.Println("Failed to create shopping list: ", err.Error())
		ctx.AbortWithStatus(http.StatusInternalServerError) 
	}

	ctx.IndentedJSON(http.StatusOK, gin.H{
		"status": "created",
		"id": shoppingListId,
	})
}

func getScraperHealth(ctx *gin.Context) {
	json, err := controller.GetScraperHealthCheck()
	core.SendResponseOrError(ctx, json, err)
}

func startScraper(ctx *gin.Context) {
	json, err := controller.StartScraper()
	core.SendResponseOrError(ctx, json, err)
}

func getScraperState(ctx *gin.Context) {
	json, err := controller.GetScraperState()
	core.SendResponseOrError(ctx, json, err)
}

func (deps *Depends) findProducts(ctx *gin.Context) {
	query := ctx.Request.URL.Query()
	var search_text string = query.Get("search_text")

	products, err := controller.FindProductInDb(deps.Database, search_text)
	core.SendResponseOrError(ctx, products, err)
}

func (deps *Depends) getProducts(ctx *gin.Context) {
	products, err := controller.GetProductsFromDb(deps.Database)
	core.SendResponseOrError(ctx, products, err)
}

func (deps *Depends) getProductById(ctx *gin.Context) {
	product, err := controller.GetProductById(deps.Database, ctx.Query("id"))
	core.SendResponseOrError(ctx, product, err)
}

func (deps *Depends) getScrapeErrors(ctx *gin.Context) {
	errors, err := controller.GetScrapeErrorsFromDb(deps.Database)
	core.SendResponseOrError(ctx, errors, err)
}

func (deps *Depends) getProductsByStoreName(ctx *gin.Context) {
	products, err := controller.GetProductsByStore(deps.Database, ctx.Query("store"))
	core.SendResponseOrError(ctx, products, err)
}