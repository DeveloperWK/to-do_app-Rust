use axum::{Router, routing::post};

use crate::{AppState, handlers};

pub fn auth_routes() -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login));
    Router::new().nest("/auth", auth_routes)
}
