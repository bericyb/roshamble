use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    Form,
};

use crate::{
    services::users_service::{self, LoginRequest, LoginResponse, NewUserRequest, SignUpResponse},
    AppState,
};

pub async fn get_sign_up(State(state): State<AppState>) -> impl IntoResponse {
    return Html(
        state
            .templates
            .render(
                "auth/register",
                &SignUpResponse {
                    message: "".to_string(),
                    token: Some("".to_string()),
                },
            )
            .unwrap()
            .into_response(),
    );
}

pub async fn sign_up(
    State(state): State<AppState>,
    Form(form): Form<NewUserRequest>,
) -> impl IntoResponse {
    let response = users_service::sign_up_user(state.pool, form).await;

    let mut headers = HeaderMap::new();
    if response.0.is_success() && response.1.token.is_some() {
        headers.insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&format!(
                "Authorization={}; HttpOnly; Secure; Path=/; SameSite=Strict",
                response.1.token.unwrap_or("".to_string())
            ))
            .unwrap(),
        );
        headers.insert("HX-Redirect", "/dashboard".parse().unwrap());
        return (StatusCode::OK, headers, "").into_response();
    } else if response.0.is_client_error() {
        return (StatusCode::BAD_REQUEST, response.1.message).into_response();
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, response.1.message).into_response();
    }
}

pub async fn get_log_in(State(state): State<AppState>) -> impl IntoResponse {
    return Html(
        state
            .templates
            .render(
                "auth/login",
                &LoginResponse {
                    message: "".to_string(),
                    token: "".to_string(),
                },
            )
            .unwrap()
            .into_response(),
    );
}

pub async fn log_in(
    State(state): State<AppState>,
    Form(form): Form<LoginRequest>,
) -> impl IntoResponse {
    let response = users_service::log_in_user(state.pool, form).await;

    let mut headers = HeaderMap::new();
    if response.0.is_success() {
        headers.insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&format!(
                "Authorization={}; HttpOnly; Secure; Path=/; SameSite=Strict",
                response.1.token
            ))
            .unwrap(),
        );
        headers.insert("HX-Redirect", "/dashboard".parse().unwrap());
        return (StatusCode::OK, headers, "").into_response();
    } else if response.0.is_client_error() {
        return (StatusCode::BAD_REQUEST, response.1.message).into_response();
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, response.1.message).into_response();
    }
}

pub async fn password_reset(State(state): State<AppState>) -> impl IntoResponse {
    return Html(
        state
            .templates
            .render("auth/password_reset", &serde_json::json!({}))
            .unwrap()
            .into_response(),
    );
}

pub async fn get_password_reset(State(state): State<AppState>) -> impl IntoResponse {
    return Html(
        state
            .templates
            .render("auth/password_reset", &serde_json::json!({}))
            .unwrap()
            .into_response(),
    );
}
