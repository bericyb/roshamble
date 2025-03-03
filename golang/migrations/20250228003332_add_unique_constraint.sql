-- +goose Up
ALTER TABLE ranked_matchmaking_queue ADD UNIQUE (username);
ALTER TABLE casual_matchmaking_queue ADD UNIQUE (username);

