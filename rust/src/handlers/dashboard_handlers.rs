use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Extension,
};
use serde_json::json;

use crate::{services::users_service::Claims, AppState};

pub async fn handle_dashboard(State(state): State<AppState>) -> impl IntoResponse {
    let data = json!({});
    let body = state.templates.render("dashboard", &data).unwrap();
    Html(body)
}

pub async fn handle_gametypes(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let data = json!({
        "player": {
            "id": claims.id,
        }
    });
    let body = state.templates.render("gametypes", &data).unwrap();
    Html(body)
}
