use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde_json::json;

use crate::AppState;

pub async fn handle_dashboard(State(state): State<AppState>) -> impl IntoResponse {
    let data = json!({});
    let body = state.templates.render("dashboard", &data).unwrap();
    Html(body)
}
