use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    AppState,
    models::todos::{CreateTodos, Todos, UpdateTodos},
};
#[axum::debug_handler]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodos>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let id = Uuid::new_v4();
    let result = sqlx::query_as::<_, Todos>(
        r#"
INSERT INTO todos (id,title,description,status,reminder,due_date,completed_at,user_id,created_at)
VALUES ($1,$2,$3,$4::status_enum,$5,$6,$7,$8,NOW())
RETURNING *
        "#,
    )
    .bind(id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.status)
    .bind(&payload.reminder)
    .bind(&payload.due_date)
    .bind(&payload.completed_at)
    .bind(&payload.user_id)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(json!({"message":"todo is created"})),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Something went wrong", "error": e.to_string()})),
        )),
    }
}
#[axum::debug_handler]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodos>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let result = sqlx::query_as::<_, Todos>(
        r#"
        UPDATE todos
        SET
            title=$1,
            description=$2,
            status=$3,
            reminder=$4,
            due_date=$5,
            completed_at=$6
        WHERE id = $7
        RETURNING *;
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.status)
    .bind(&payload.reminder)
    .bind(&payload.due_date)
    .bind(&payload.completed_at)
    .bind(id)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(_) => Ok((StatusCode::OK, Json(json!({"message":"todo updated"})))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Something went wrong", "error": e.to_string()})),
        )),
    }
}

#[axum::debug_handler]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let result = sqlx::query!(
        r#"
    DELETE FROM todos
    WHERE
        id = $1
        "#,
        id
    )
    .execute(&state.db)
    .await;
    match result {
        Ok(_) => Ok((StatusCode::OK, Json(json!({"message":"Deleted Success"})))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Something went wrong", "error": e.to_string()})),
        )),
    }
}
pub async fn get_all_todos(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match sqlx::query_as::<_, Todos>("SELECT * FROM todos")
        .fetch_all(&state.db)
        .await
    {
        Ok(todos) => {
            if todos.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({
                          "message": "No todos found"
                    })),
                ));
            }
            Ok((StatusCode::OK, Json(todos)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Something went wrong",
                "error": e.to_string()
            })),
        )),
    }
}
pub async fn get_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match sqlx::query_as::<_, Todos>("SELECT * FROM todos where id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(sqlx::Error::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Todo not found" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Something went wrong",
                "error": e.to_string()
            })),
        )),
    }
}
