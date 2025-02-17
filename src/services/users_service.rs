use axum::http::StatusCode;
use jsonwebtoken as jwt;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewUserRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SignUpResponse {
    pub message: String,
    pub token: Option<String>,
}

pub async fn sign_up_user(
    pool: Pool<Postgres>,
    body: NewUserRequest,
) -> (StatusCode, SignUpResponse) {
    let hashed_password = bcrypt::hash(body.password.clone(), 12).unwrap();
    let res = sqlx::query!(
        "INSERT INTO users (username, password, email) VALUES ($1, $2, $3);",
        &body.username,
        hashed_password,
        &body.email
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => {
            let res = sqlx::query!(
                "SELECT id, username, email, password FROM users WHERE email = $1",
                &body.email,
            )
            .fetch_one(&pool)
            .await;

            match res {
                Ok(user) => {
                    if bcrypt::verify(body.password, &user.password).unwrap() {
                        // Set expiration time to 1 year
                        let claims = Claims {
                            sub: 0,
                            username: user.username.clone(),
                            email: user.email,
                            id: user.id.to_string(),
                            exp: (chrono::Utc::now() + chrono::Duration::days(365)).timestamp()
                                as usize,
                        };
                        match jwt::encode(
                            &jwt::Header::default(),
                            &claims,
                            &jwt::EncodingKey::from_secret(
                                dotenv::var("SECRET").unwrap().as_bytes(),
                            ),
                        ) {
                            Ok(t) => {
                                return (
                                    StatusCode::OK,
                                    SignUpResponse {
                                        message: format!("Welcome {}", user.username),
                                        token: Some(t),
                                    },
                                )
                            }
                            Err(e) => {
                                tracing::error!("error creating jwt {}", e.to_string());
                                return (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        SignUpResponse {
                                            message: "There was an error. But your account was created. Please try logging in again later..."
                                                .to_string(),
                                            token: None,
                                        },
                                    );
                            }
                        };
                    } else {
                        return (
                            StatusCode::UNAUTHORIZED,
                            SignUpResponse {
                                message: "Invalid username or password".to_string(),
                                token: Some("".to_string()),
                            },
                        );
                    }
                }
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        return (
                            StatusCode::UNAUTHORIZED,
                            SignUpResponse {
                                message: "Invalid username or password".to_string(),
                                token: None,
                            },
                        )
                    }
                    _ => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            SignUpResponse {
                                message: "There was an error logging in. Please try again later..."
                                    .to_string(),
                                token: None,
                            },
                        )
                    }
                },
            }
        }
        Err(e) => match e {
            sqlx::Error::Database(ref db_err) => {
                if db_err.is_unique_violation() {
                    return (
                        StatusCode::CONFLICT,
                        SignUpResponse {
                            message: "Email or username already exists".to_string(),
                            token: None,
                        },
                    );
                }
                tracing::error!("{:?}", db_err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SignUpResponse {
                        message: "There was an error signing up. Please try again later..."
                            .to_string(),
                        token: None,
                    },
                );
            }
            db_err => {
                tracing::error!("{:?}", db_err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SignUpResponse {
                        message: "There was an error signing up. Please try again later..."
                            .to_string(),
                        token: None,
                    },
                );
            }
        },
    }
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    username_or_email: String,
    password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Claims {
    sub: i32,
    pub username: String,
    pub id: String,
    email: String,
    exp: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

pub async fn log_in_user(pool: Pool<Postgres>, body: LoginRequest) -> (StatusCode, LoginResponse) {
    let res = sqlx::query!(
        "SELECT id, username, email, password FROM users WHERE (username = $1 or email = $1);",
        &body.username_or_email,
    )
    .fetch_one(&pool)
    .await;

    match res {
        Ok(user) => {
            if bcrypt::verify(body.password, &user.password).unwrap() {
                // Expiration time is 1 year from now
                let claims = Claims {
                    sub: 0,
                    username: user.username.clone(),
                    id: user.id.to_string(),
                    email: user.email,
                    exp: (chrono::Utc::now() + chrono::Duration::days(365)).timestamp() as usize,
                };
                let token = match jwt::encode(
                    &jwt::Header::default(),
                    &claims,
                    &jwt::EncodingKey::from_secret(dotenv::var("SECRET").unwrap().as_bytes()),
                ) {
                    Ok(t) => t,
                    Err(e) => {
                        tracing::error!("error creating jwt {}", e.to_string());
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            LoginResponse {
                                message: "There was an error logging in. Please try again later..."
                                    .to_string(),
                                token: "".to_string(),
                            },
                        );
                    }
                };

                return (
                    StatusCode::OK,
                    LoginResponse {
                        message: format!("Welcome {}", user.username),
                        token,
                    },
                );
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    LoginResponse {
                        message: "Invalid username or password".to_string(),
                        token: "".to_string(),
                    },
                );
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                return (
                    StatusCode::UNAUTHORIZED,
                    LoginResponse {
                        message: "Invalid username or password".to_string(),
                        token: "".to_string(),
                    },
                )
            }
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    LoginResponse {
                        message: "There was an error logging in. Please try again later..."
                            .to_string(),
                        token: "".to_string(),
                    },
                )
            }
        },
    };
}
