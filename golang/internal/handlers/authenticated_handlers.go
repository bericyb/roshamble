package handlers

import (
	"fmt"
	"log/slog"
	"net/http"
	"roshamble/internal/services"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt"
)

func getClaims(c *gin.Context) (services.Claims, error) {
	claimsValue, ok := c.Get("claims")
	if !ok {
		fmt.Println("error getting claims")
		return services.Claims{}, fmt.Errorf("error getting claims")
	}
	claims, ok := claimsValue.(jwt.MapClaims)
	if !ok {
		fmt.Println("error converting claims")
		return services.Claims{}, fmt.Errorf("error getting claims")
	}

	eloFloat, ok := claims["elo"].(float64)
	if !ok {
		return services.Claims{}, fmt.Errorf("error getting claims")
	}
	eloInt := int(eloFloat)

	structClaims := services.Claims{
		ID:       claims["id"].(string),
		Username: claims["username"].(string),
		Email:    claims["email"].(string),
		Elo:      eloInt,
	}
	return structClaims, nil
}

func (h *Handler) GetDashboard(c *gin.Context) {
	c.HTML(http.StatusOK, "dashboard.html", gin.H{})
}

func (h *Handler) GetGametypes(c *gin.Context) {
	c.HTML(http.StatusOK, "gametypes.html", gin.H{})
}

func (h *Handler) GetMatchmaking(c *gin.Context) {
	claims, err := getClaims(c)
	if err != nil {
		c.HTML(http.StatusOK, "gametypes.html", gin.H{"message": "Error with player identity. Please try logging out and back in again."})
		return
	}

	title := "Ranked"
	if c.Param("mode") != "ranked" {
		title = "Casual"
		err := services.AddPlayerToRankedMatchmaking(c, h.DB, claims)
		if err != nil {
			slog.Error(err.Error())
			c.HTML(http.StatusOK, "gametypes.html", gin.H{"message": "Error joining matchmaking queue"})
			return
		}
	} else {
		err := services.AddPlayerToRankedMatchmaking(c, h.DB, claims)
		if err != nil {
			slog.Error(err.Error())
			c.HTML(http.StatusOK, "gametypes.html", gin.H{"message": "Error joining matchmaking queue"})
			return
		}
	}

	c.HTML(http.StatusOK, "matchmaking.html", gin.H{"mode": c.Param("mode"), "title": title, "username": claims.Username})
	return
}

func (h *Handler) GetMatchmakingCount(c *gin.Context) {
	mode := c.Param("mode")
	count, err := services.GetMatchmakingCount(h.DB, mode)
	if err != nil {
		slog.Error(err.Error())
		c.HTML(http.StatusOK, "playercount.html", gin.H{"count: ": "?", "message": "There waas a problem getting the number of queued players"})
	}
	c.HTML(http.StatusOK, "playercount.html", gin.H{"count": count})
}

func (h *Handler) GetMatchmakingReady(c *gin.Context) {
	mode := c.Param("mode")
	match := services.FindMatchmakingPair(h.DB, mode, getClaims(c))
	c.HTML(http.StatusOK, "empty.html", gin.H{})
}

func (h *Handler) LeaveMatchmaking(c *gin.Context) {
	c.HTML(http.StatusOK, "gametypes.html", gin.H{})
}
