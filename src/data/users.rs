use axum::http::StatusCode;
use jsonwebtoken as jwt;
use sqlx::{Pool, Postgres};

#[derive(serde::Deserialize)]
pub struct NewUserRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SignUpResponse {
    pub message: String,
}

pub async fn sign_up_user(
    pool: Pool<Postgres>,
    body: NewUserRequest,
) -> (StatusCode, SignUpResponse) {
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
            SignUpResponse {
                message: format!("Welcome {}", body.username),
            },
        ),
        Err(e) => match e {
            sqlx::Error::Database(ref db_err) => {
                if db_err.is_unique_violation() {
                    return (
                        StatusCode::CONFLICT,
                        SignUpResponse {
                            message: "Email or username already exists".to_string(),
                        },
                    );
                }
                tracing::error!("{:?}", db_err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SignUpResponse {
                        message: "There was an error signing up. Please try again later..."
                            .to_string(),
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims {
    sub: i32,
    username: String,
    email: String,
    exp: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

pub async fn log_in_user(pool: Pool<Postgres>, body: LoginRequest) -> (StatusCode, LoginResponse) {
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
