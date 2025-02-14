use axum::{routing::get, Router};

use crate::{handlers::dashboard_handlers, AppState};

// Routes for authenticated users
pub fn add_routes() -> Router<AppState> {
    return Router::new().route("/dashboard", get(dashboard_handlers::handle_dashboard));
}
