use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct NewUserRequest {
    username: String,
    password: String,
    email: String,
}

struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
}

pub async fn sign_up(
    State(pool): State<PgPool>,
    Json(body): Json<NewUserRequest>,
) -> impl IntoResponse {
    let hashed_password = bcrypt::hash(body.password, 12).unwrap();
    let res = sqlx::query("INSERT INTO users (username, password, email) VALUES ($1, $2, $3);")
        .bind(&body.username)
        .bind(hashed_password)
        .bind(&body.email)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": format!("User created: {}", body.username) })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn log_in(
    State(pool): State<PgPool>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let hashed_password = bcrypt::hash(body.password, 12).unwrap();
    let res = sqlx::query("SELECT * FROM users WHERE username = $1 AND password = $2;")
        .bind(&body.username)
        .bind(hashed_password)
        .execute(&pool)
        .await;
}
