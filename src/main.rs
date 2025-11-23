mod db;
mod handlers;
mod middlewares;
mod models;
mod routers;
mod utils;

use std::env;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    db::{DBpool, connect_db},
    routers::todos::todos_routes,
};

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
        .route("/login", post(handlers::auth::login));

    let protected_routes = Router::new()
        .route("/profile", get(handlers::user::get_profile))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth::auth_middleware,
        ));
    let todos = Router::new()
        .merge(todos_routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth::auth_middleware,
        ));
    let app = Router::new()
        .nest("/auth", public_routes)
        .nest("/api", protected_routes)
        .nest("/api", todos)
        .layer(CorsLayer::permissive())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
