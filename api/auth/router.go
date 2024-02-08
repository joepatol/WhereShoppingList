package auth

import (
	"core"
	"models"
	"net/http"

	"github.com/gin-gonic/gin"
)

type Depends core.Depends

func Add(base gin.IRouter, cdeps core.Depends) {
	router := base.Group("/auth")
	deps := Depends { cdeps.Database, cdeps.Logger }

	router.POST("/register", deps.Register)
	router.POST("/login", deps.Login)
	router.Use(JwtAuthMiddleware())
	router.GET("/user", deps.GetUser)
}

func (deps *Depends) GetUser(ctx *gin.Context) {
	userId, err := ExtractTokenID(ctx)

	if err != nil {
		ctx.JSON(http.StatusUnauthorized, gin.H{"error": err.Error()})
		return
	}

	user, err := getUserById(userId, deps.Database)

	if err != nil {
		ctx.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	ctx.JSON(http.StatusOK, gin.H{"message": "success","data": user})
}

func (deps *Depends) Login(ctx *gin.Context) {
	var input LoginInput

	if err := ctx.ShouldBindJSON(&input); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	token, err := loginCheck(input.Email, input.Password, deps.Database)

	if err != nil {
		ctx.JSON(http.StatusUnauthorized, gin.H{"error": "email or password is incorrect."})
		return
	}

	ctx.JSON(http.StatusOK, gin.H{"token": token})
}

func (deps *Depends) Register(ctx *gin.Context) {
	var input RegisterInput

	if err := ctx.ShouldBindJSON(&input); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	user := models.User{}

	user.FirstName = input.FirstName
	user.LastName = input.LastName
	user.Email = input.Email
	user.Password = input.Password

	_, err := saveUser(user, deps.Database)

	if err != nil{
		ctx.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	ctx.JSON(http.StatusOK, gin.H{"message": "registration success"})
}
