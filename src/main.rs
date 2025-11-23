mod db;
mod handlers;
mod jwt_service;
mod middlewares;
mod models;
mod utils;

use std::env;

use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::db::{DBpool, connect_db};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DBpool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = connect_db().await;
    let state = AppState {
        db: pool,
        jwt_secret,
    };
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter("DEBUG,sqlx=warn")
        .init();
    let public_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/todos", get(handlers::todos::get_all_todos))
        .route("/todos/{id}", get(handlers::todos::get_todo_by_id))
        .route("/todos", post(handlers::todos::create_todo))
        .route("/todos/{id}", patch(handlers::todos::update_todo))
        .route("/todos/{id}", delete(handlers::todos::delete_task));

    let protected_routes = Router::new()
        .route("/profile", get(handlers::user::get_profile))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth::auth_middleware,
        ));

    let app = Router::new()
        .nest("/auth", public_routes)
        .nest("/api", protected_routes)
        .layer(CorsLayer::permissive())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
