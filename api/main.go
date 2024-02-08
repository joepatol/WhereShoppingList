package main

import (
	"v1"
	"net/http"
	"auth"
	"db"
	"log"
	"core"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
)

func main() {
	godotenv.Load(".env")
    router := gin.Default()

	database := db.ConnectDb()
	db.Migrate(database)

	deps := core.Depends{ 
		Database: database,
		Logger: log.Default(),
	}

	router.GET("/health", healthCheck)
	v1.Add(router, deps)
	auth.Add(router, deps)
	
    router.Run("0.0.0.0:8080")
}

func healthCheck(ctx *gin.Context) {
	ctx.IndentedJSON(http.StatusOK, "Hello there")
}