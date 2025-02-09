use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use jsonwebtoken as jwt;
use serde_json::json;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct NewUserRequest {
    username: String,
    password: String,
    email: String,
}

pub async fn sign_up(
    State(pool): State<PgPool>,
    Json(body): Json<NewUserRequest>,
) -> impl IntoResponse {
    let hashed_password = bcrypt::hash(body.password, 12).unwrap();
    let res = sqlx::query!(
        "INSERT INTO users (username, password, email) VALUES ($1, $2, $3);",
        &body.username,
        hashed_password,
        &body.email
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": format!("User created: {}", body.username) })),
        )
            .into_response(),
        Err(e) => match e {
            sqlx::Error::Database(ref db_err) => {
                if db_err.is_unique_violation() {
                    return (
                        StatusCode::CONFLICT,
                        Json(json!({ "error": "Username or email already exists" })),
                    )
                        .into_response();
                }
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                )
                    .into_response();
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response(),
        },
    }
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    username_or_email: String,
    password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims {
    sub: i32,
    username: String,
    email: String,
    exp: usize,
}

pub async fn log_in(
    State(pool): State<PgPool>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let hashed_password = bcrypt::hash(body.password.clone(), 12).unwrap();
    let res = sqlx::query!(
        "SELECT username, email, password FROM users WHERE (username = $1 or email = $1) AND password = $2;",
        &body.username_or_email,
        hashed_password
    )
    .fetch_one(&pool)
    .await;

    match res {
        Ok(user) => {
            if bcrypt::verify(body.password, &user.password).unwrap() {
                let claims = Claims {
                    sub: 0,
                    username: user.username.clone(),
                    email: user.email,
                    exp: 0,
                };
                let token = jwt::encode(
                    &jwt::Header::default(),
                    &claims,
                    &jwt::EncodingKey::from_secret(dotenv::var("SECRET").unwrap().as_bytes()),
                )
                .unwrap();
                return (
                    StatusCode::OK,
                    Json(
                        json!({ "message": format!("Welcome {}", user.username), "token": token }),
                    ),
                )
                    .into_response();
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Invalid username or password" })),
                )
                    .into_response();
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Invalid username or password" })),
                )
                    .into_response()
            }
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                )
                    .into_response()
            }
        },
    };
}

fn render_auth_page(error_msg: string) -> String {}
