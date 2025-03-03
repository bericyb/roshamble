package main

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"
	"roshamble/internal/auth"
	"roshamble/internal/handlers"

	"github.com/gin-gonic/gin"
	_ "github.com/lib/pq"
	"github.com/pressly/goose/v3"
)

var db *sql.DB

func main() {
	initDB()
	r := gin.Default()

	// Load templates
	r.LoadHTMLGlob("templates/**/*")

	// Serve static files
	r.Static("/assets", "./assets")

	handler := &handlers.Handler{DB: db}

	r.GET("/", handler.Root)
	r.GET("/ping", handler.PingHandler)
	r.GET("/empty", handler.Empty)
	r.GET("/auth/login", handler.GetLogin)
	r.POST("/auth/login", handler.Login)
	r.GET("/auth/logout", handler.GetLogout)
	r.GET("/auth/register", handler.GetRegister)
	r.POST("/auth/register", handler.Register)
	r.GET("/auth/reset", handler.GetResetPassword)

	// Protected routes
	auth := r.Group("/").Use(auth.JwtAuthMiddleware())
	auth.GET("/protected", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"message": "You are authenticated"})
	})
	auth.GET("/dashboard", handler.GetDashboard)
	auth.GET("/gametypes", handler.GetGametypes)
	auth.GET("/matchmaking/:mode", handler.GetMatchmaking)
	auth.GET("/matchmaking/ready/:mode/:username", handler.GetMatchmakingReady)
	auth.GET("/matchmaking/:mode/count", handler.GetMatchmakingCount)
	auth.POST("/remove/:username/matchmaking/:mode", handler.LeaveMatchmaking)

	port := ":8000"
	fmt.Printf("Server running at http://localhost%s\n", port)
	r.Run(port)
}

func initDB() {
	var err error
	connStr := "postgres://postgres:password@localhost:5432/postgres?sslmode=disable"
	db, err = sql.Open("postgres", connStr)
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}

	db.SetMaxOpenConns(25)
	db.SetMaxIdleConns(25)
	db.SetConnMaxLifetime(0)

	if err := db.Ping(); err != nil {
		log.Fatalf("Database ping failed: %v", err)
	}

	applyMigrations()
}

func applyMigrations() {
	migrationsDir := "migrations"
	if err := goose.Up(db, migrationsDir); err != nil {
		log.Fatalf("Failed to apply migrations: %v", err)
	}
	log.Println("Database migrations applied successfully.")
}
