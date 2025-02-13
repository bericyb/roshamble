use axum::{
    body::Body,
    extract::{Request, State},
    http::Response,
    middleware::{from_fn, Next},
    response::{Html, IntoResponse, Redirect},
    routing::any,
    Router,
};
use data::users::Claims;
use handlebars::{DirectorySourceOptions, Handlebars};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{sync::Arc, time::Duration};

mod data;
mod handlers;
mod routes;

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
        .layer(from_fn(auth_middleware))
        .merge(routes::auth_routes::add_routes())
        .merge(Router::new().nest_service("/assets", ServeDir::new("assets")))
        .route("/", any(serve_index))
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
async fn auth_middleware(req: Request, next: Next) -> impl IntoResponse {
    let secret = dotenv::var("SECRET").unwrap();
    if let Some(header) = req.headers().get("Authorization") {
        match header.to_str() {
            Ok(auth) => {
                if validate_jwt(auth, &secret).is_ok() {
                    return next.run(req).await;
                }
            }
            Err(_) => {
                return Redirect::temporary("/").into_response();
            }
        }
    }
    tracing::debug!("No Authorization header found");
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
