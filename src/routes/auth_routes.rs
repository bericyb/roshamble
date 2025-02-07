use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

use crate::handlers::auth_handlers;

pub fn add_routes() -> Router<Pool<Postgres>> {
    return Router::new()
        .route("/auth/register", post(auth_handlers::sign_up))
        .route("/auth/login", post(auth_handlers::log_in));
}
