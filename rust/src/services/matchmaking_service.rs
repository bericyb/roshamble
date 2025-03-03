use sqlx::{Pool, Postgres};

use super::users_service::Claims;

pub enum GameType {
    Ranked,
    Casual,
    Tournament,
}

pub async fn add_player_to_ranked_queue(pool: Pool<Postgres>, player: Claims, gamemode: GameType) {
    // // Add player to queue
    // let res = sqlx::query!(
    //     "INSERT INTO ranked_matchmaking_queue (player_id, skill_rating) VALUES ($1, $2);",
    //     &player.id,
    //     &player.elo,
    // )
    // .execute(&pool)
    // .await;

    // match res {
    //     Ok(_) => {
    //         println!("Player added to queue");
    //     }
    //     Err(e) => {
    //         println!("Error adding player to queue: {}", e);
    //     }
    // }
}

pub async fn add_player_to_casual_queue(pool: Pool<Postgres>, player: Claims, gamemode: GameType) {
    // Add player to queue
    // let res = sqlx::query!(
    //     "INSERT INTO ranked_matchmaking_queue (player_id, skill_rating) VALUES ($1, $2);",
    //     &player.id,
    //     &player.elo,
    // )
    // .execute(&pool)
    // .await;

    // match res {
    //     Ok(_) => {
    //         println!("Player added to queue");
    //     }
    //     Err(e) => {
    //         println!("Error adding player to queue: {}", e);
    //     }
    // }
}
pub async fn check_player_match(player_id: String) {
    // Check if player has a match
    // ...
}
