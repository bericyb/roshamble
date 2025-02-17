use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde_json::json;

use crate::{
    services::matchmaking_service::{self, GameType},
    AppState,
};

pub async fn handle_ranked(
    Path(player_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let data = json!({
        "title": "Ranked",
        "player": {
            "id": player_id
        }
    });
    matchmaking_service::add_player_to_queue(player_id, GameType::Ranked).await;
    let body = state.templates.render("matchmaking", &data).unwrap();
    return Html(body);
}

pub async fn handle_casual(
    Path(player_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let data = json!({
        "title": "Casual",
        "player": {
                    "id": player_id
                }
    });
    matchmaking_service::add_player_to_queue(player_id, GameType::Casual).await;
    let body = state.templates.render("matchmaking", &data).unwrap();
    return Html(body);
}

pub async fn handle_ready(
    Path(player_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    matchmaking_service::check_player_match(player_id).await;
    return Html("Waiting for a match...");
}

pub async fn handle_count(State(state): State<AppState>) -> impl IntoResponse {
    return Html("14 players online");
}
