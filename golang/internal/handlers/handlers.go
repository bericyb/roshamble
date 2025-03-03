package handlers

import (
	"database/sql"
	"net/http"
	"roshamble/internal/services"

	"github.com/gin-gonic/gin"
)

type Handler struct {
	DB *sql.DB
}

func (h *Handler) Root(c *gin.Context) {
	cookie, err := c.Cookie("Authorization")
	if err != nil || cookie != "" {
		http.Redirect(c.Writer, c.Request, "/dashboard", 302)
		return
	}
	c.HTML(http.StatusOK, "index.html", gin.H{})
}

func (h *Handler) Empty(c *gin.Context) {
	c.HTML(http.StatusOK, "empty.html", gin.H{})
}

func (h *Handler) GetLogin(c *gin.Context) {
	cookie, err := c.Cookie("Authorization")
	if err != nil || cookie != "" {
		http.Redirect(c.Writer, c.Request, "/dashboard", 302)
		return
	}
	c.HTML(http.StatusOK, "login.html", gin.H{})
}

func (h *Handler) Login(c *gin.Context) {
	token, errMessage := services.LoginUser(c, h.DB)
	if errMessage != "" {
		c.HTML(http.StatusOK, "login.html", gin.H{"message": errMessage})
	}

	c.SetCookie("Authorization", token, 3600*24*365, "/", "", false, true)
	c.Header("HX-Redirect", "/dashboard")
	c.JSON(http.StatusOK, gin.H{})
}

func (h *Handler) GetRegister(c *gin.Context) {
	if cookie, err := c.Cookie("Authorization"); err != nil || cookie != "" {
		http.Redirect(c.Writer, c.Request, "/dashboard", 302)
		return
	}
	c.HTML(http.StatusOK, "register.html", gin.H{})
}

func (h *Handler) Register(c *gin.Context) {
	token, err := services.RegisterUser(c, h.DB)
	if err != "" {
		c.HTML(http.StatusOK, "register.html", gin.H{"message": err})
	}

	c.SetCookie("Authorization", token, 3600*24*365, "/", "", false, true)
	c.Header("HX-Redirect", "/dashboard")
	c.JSON(http.StatusOK, gin.H{})
}

func (h *Handler) GetResetPassword(c *gin.Context) {
	c.HTML(http.StatusOK, "password_reset.html", gin.H{})
}

func (h *Handler) GetLogout(c *gin.Context) {
	c.SetCookie("Authorization", "", 0, "/", "", false, true)
	c.HTML(http.StatusOK, "index.html", gin.H{})
}

func (h *Handler) PingHandler(c *gin.Context) {
	c.JSON(200, gin.H{"message": "pong"})
}
