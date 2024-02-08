package core

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

func SendResponseOrError (ctx *gin.Context, obj any, err error) {
	if err != nil {
		ctx.IndentedJSON(http.StatusInternalServerError, err.Error())
	} else {
		ctx.IndentedJSON(http.StatusOK, obj)
	}
}
