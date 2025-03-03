use axum::{routing::get, Router};

use crate::{
    handlers::{dashboard_handlers, matchmaking_handlers},
    AppState,
};

// Routes for authenticated users
pub fn add_routes() -> Router<AppState> {
    return Router::new()
        .route("/dashboard", get(dashboard_handlers::handle_dashboard))
        .route("/gametypes", get(dashboard_handlers::handle_gametypes))
        .route(
            "/matchmaking/ranked/{playerid}",
            get(matchmaking_handlers::handle_ranked),
        )
        .route(
            "/matchmaking/casual/{playerid}",
            get(matchmaking_handlers::handle_casual),
        )
        .route(
            "/matchmaking/{gamemode}/count",
            get(matchmaking_handlers::handle_count),
        )
        .route(
            "/matchmaking/ready/{playerid}",
            get(matchmaking_handlers::handle_ready),
        );
}
