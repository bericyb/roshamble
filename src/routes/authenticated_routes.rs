use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde_json::json;

use crate::AppState;

pub fn add_routes() -> Router<AppState> {
    return Router::new().route("/dashboard", get(handle_dashboard));
}

async fn handle_dashboard(State(state): State<AppState>) -> impl IntoResponse {
    let data = json!({});
    let body = state.templates.render("dashboard", &data).unwrap();
    Html(body)
}
