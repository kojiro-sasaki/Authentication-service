package routes

import (
	"auth-service/handlers"
	"github.com/gin-gonic/gin"
)

func SetupRoutes(r *gin.Engine) {

	r.POST("/register", handlers.Register)
	r.POST("/login", handlers.Login)

}
