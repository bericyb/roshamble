-- +goose UP
ALTER TABLE ranked_matchmaking_queue DROP COLUMN queue_id, DROP COLUMN player_id;
ALTER TABLE ranked_matchmaking_queue ADD COLUMN username VARCHAR(255);
ALTER TABLE ranked_matchmaking_queue ADD COLUMN id SERIAL PRIMARY KEY;

ALTER TABLE casual_matchmaking_queue DROP COLUMN queue_id, DROP COLUMN player_id;
ALTER TABLE casual_matchmaking_queue ADD COLUMN id SERIAL PRIMARY KEY;
ALTER TABLE casual_matchmaking_queue ADD COLUMN username VARCHAR(255);
