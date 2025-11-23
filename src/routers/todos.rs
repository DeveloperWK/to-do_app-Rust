use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    AppState,
    handlers::todos::{create_todo, delete_todo, get_all_todos, get_todo_by_id, update_todo},
};

pub fn todos_routes() -> Router<AppState> {
    let todos_routes = Router::new()
        .route("/", get(get_all_todos))
        .route("/{id}", get(get_todo_by_id))
        .route("/", post(create_todo))
        .route("/{id}", patch(update_todo))
        .route("/{id}", delete(delete_todo));
    Router::new().nest("/todos", todos_routes)
}
