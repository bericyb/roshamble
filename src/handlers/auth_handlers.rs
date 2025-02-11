use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Form,
};

use crate::{
    data::users::{self, LoginRequest, LoginResponse, NewUserRequest, SignUpResponse},
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
    let response = users::sign_up_user(state.pool, form).await;

    if response.0.is_success() {
        return Html(
            state
                .templates
                .render("auth/login", &response.1)
                .unwrap()
                .into_response(),
        );
    }
    if !response.0.is_success() {
        return Html(
            state
                .templates
                .render("auth/register", &response.1)
                .unwrap()
                .into_response(),
        );
    }
    return Html(
        state
            .templates
            .render("auth/register", &response.1)
            .unwrap()
            .into_response(),
    );
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
    let response = users::log_in_user(state.pool, form).await;

    if response.0.is_success() {
        return Html(
            state
                .templates
                .render("dashboard", &response.1)
                .unwrap()
                .into_response(),
        );
    } else {
        return Html(
            state
                .templates
                .render("auth/login", &response.1)
                .unwrap()
                .into_response(),
        );
    }
}
