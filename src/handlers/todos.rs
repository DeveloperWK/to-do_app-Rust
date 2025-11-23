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

    let _ = sqlx::query_as::<_, Todos>(
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
    .await
    .unwrap();
    Ok((
        StatusCode::CREATED,
        Json(json!({"message":"todo is created"})),
    ))
}
#[axum::debug_handler]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodos>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    println!("{}", id);
    let _ = sqlx::query_as::<_, Todos>(
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
    .await
    .unwrap();
    Ok((StatusCode::OK, Json(json!({"message":"todo updated"}))))
}

#[axum::debug_handler]
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let _ = sqlx::query!(
        r#"
    DELETE FROM todos 
    WHERE 
        id = $1
        "#,
        id
    )
    .execute(&state.db)
    .await;
    Ok((StatusCode::OK, Json(json!({"message":"Deleted Success"}))))
}
pub async fn get_all_todos(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let todos = sqlx::query_as::<_, Todos>("SELECT * FROM todos")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Ok(Json(todos))
}
pub async fn get_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let todo = sqlx::query_as::<_, Todos>("SELECT * FROM todos where id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .unwrap();
    Ok(Json(todo))
}
