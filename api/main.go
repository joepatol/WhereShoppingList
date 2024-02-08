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
	err := godotenv.Load(".env")

	if err != nil {
		log.Fatalf("Error loading .env file")
	}	

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
	
    router.Run("localhost:8080")
}

func healthCheck(ctx *gin.Context) {
	ctx.IndentedJSON(http.StatusOK, "Hello there")
}