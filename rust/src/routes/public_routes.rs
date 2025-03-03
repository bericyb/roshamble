use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::auth_handlers, AppState};

pub fn add_routes() -> Router<AppState> {
    return Router::new()
        .route("/auth/register", post(auth_handlers::sign_up))
        .route("/auth/register", get(auth_handlers::get_sign_up))
        .route("/auth/login", post(auth_handlers::log_in))
        .route("/auth/login", get(auth_handlers::get_log_in))
        .route("/auth/password_reset", post(auth_handlers::password_reset))
        .route(
            "/auth/password_reset",
            get(auth_handlers::get_password_reset),
        );
}
