use axum::{
    body::Body,
    extract::{Request, State},
    http::Response,
    middleware::{from_fn, Next},
    response::{Html, IntoResponse, Redirect},
    routing::any,
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use handlebars::{DirectorySourceOptions, Handlebars};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use services::users_service::Claims;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{sync::Arc, time::Duration};

mod handlers;
mod routes;
mod services;

#[derive(Clone)]
struct AppState {
    pub templates: Arc<Handlebars<'static>>,
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("can't run migrations");

    let mut handlebars = Handlebars::new();

    handlebars.set_dev_mode(dotenv::var("ENVIRONMENT").unwrap() == "development");

    let mut options = DirectorySourceOptions::default();
    options.tpl_extension = ".html".to_string();

    handlebars
        .register_templates_directory("templates/", options)
        .expect("Failed to register templates");

    let app_state = AppState {
        templates: Arc::new(handlebars),
        pool,
    };

    let app = Router::new()
        .merge(routes::authenticated_routes::add_routes())
        .route("/", any(serve_index))
        .layer(from_fn(auth_middleware))
        .merge(routes::public_routes::add_routes())
        .merge(Router::new().nest_service("/assets", ServeDir::new("assets")))
        .with_state(app_state);

    // run it with hyper
    let listener = TcpListener::bind("127.0.0.1:4000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index(State(state): State<AppState>) -> Html<Response<Body>> {
    return Html(
        state
            .templates
            .render("index", &serde_json::json!({}))
            .unwrap()
            .into_response(),
    );
}

// Middleware to check Authorization header
async fn auth_middleware(cookies: CookieJar, req: Request, next: Next) -> impl IntoResponse {
    let secret = dotenv::var("SECRET").unwrap();
    cookies.iter().for_each(|cookie| {
        tracing::debug!("Cookie: {}={}", cookie.name(), cookie.value());
    });
    if let Some(auth_cookie) = cookies.get("Authorization") {
        match validate_jwt(auth_cookie.value(), &secret) {
            Ok(_) => {
                if req.uri().path() == "/" {
                    return Redirect::temporary("/dashboard").into_response();
                }
                return next.run(req).await;
            }
            Err(e) => {
                let new_jar = cookies.remove(Cookie::from("Authorization"));
                return (new_jar, Redirect::temporary("/")).into_response();
            }
        }
    } else if req.uri().path() == "/" {
        return next.run(req).await;
    }
    return Redirect::temporary("/").into_response();
}

fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
