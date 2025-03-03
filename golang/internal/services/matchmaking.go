package services

import (
	"database/sql"
	"fmt"
	"time"

	"github.com/gin-gonic/gin"
)

func AddPlayerToRankedMatchmaking(c *gin.Context, db *sql.DB, claims Claims) error {
	_, err := db.Exec("INSERT INTO ranked_matchmaking_queue (username, skill_rating, queue_time) VALUES ($1, $2, $3) ON CONFLICT (username) DO UPDATE SET queue_time = EXCLUDED.queue_time;", claims.Username, claims.Elo, time.Now())
	if err != nil {
		return fmt.Errorf("%s, error joining ranked matchmaking queue", err.Error())
	}
	return nil
}

func AddPlayerToCasualMatchmaking(c *gin.Context, db *sql.DB, claims Claims) error {
	_, err := db.Exec("INSERT INTO casual_matchmaking_queue (username, queue_time) VALUES ($1, $2, $3) ON CONFLICT (username) DO UPDATE SET queue_time = EXCLUDED.queue_time;", claims.Username, time.Now())
	if err != nil {
		return fmt.Errorf("%s, error joining casual matchmaking queue", err.Error())
	}
	return nil
}

func GetMatchmakingCount(db *sql.DB, mode string) (int, error) {
	if mode == "ranked" {
		row := db.QueryRow("SELECT COUNT(*) FROM ranked_matchmaking_queue WHERE queue_time > $1", time.Now().Add(-time.Hour))

		count := 0
		err := row.Scan(&count)
		return count, err
	} else {
		row := db.QueryRow("SELECT COUNT(*) FROM casual_matchmaking_queue WHERE queue_time > $1", time.Now().Add(-time.Hour))

		count := 0
		err := row.Scan(&count)
		return count, err
	}
}

func FindMatchmakingPair(db *sql.DB, mode string, claims Claims) (string, error) {

	// Open transaction
	tx, err := db.Begin()
	if err != nil {
		return "", err
	}
	// First check if the player's queue row is set as ready or not
	gameID := ""
	res := tx.QueryRow(fmt.Sprintf("SELECT game_id FROM %s_matchmaking_queue WHERE username = $1", mode), claims.Username)

	res.Scan(&gameID)

	// If ready return the game data etc.
	if gameID != "" {
		tx.Exec(fmt.Sprintf("DELETE FROM %s_matchmaking_queue WHERE username = $1", mode), claims.Username)
		err := tx.Commit()
		if err != nil {
			return "", err
		}
		return gameID, nil
	} else {
		// If not ready, look for another player in range
		if mode == "ranked" {
			tx.QueryRow("SELECT FOR UPDATE")
		}

		// Use SELECT FOR UPDATE then update those rows

		// If found, set player and corresponding player to ready and return game data
		insert, err := tx.Exec("INSERT INTO games (player_1, player_2, mode) VALUES ($1, $2, mode)")
		return gameID, nil
	}

	// If not ready, return nothing

	return "", nil

}
