use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        auth::{AuthResponse, LoginRequest, RegisterUser},
        user::User,
    },
    utils::{
        jwt::create_token,
        password::{hash_password, verify_password},
    },
};

#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<Value>)> {
    let existing_user =
        query_as::<_, User>("SELECT * FROM users WHERE username = $1 OR email = $2")
            .bind(&payload.username)
            .bind(&payload.email)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Database error: {}", e)})),
                )
            })?;
    if existing_user.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "Username or email already exists"})),
        ));
    }
    let hash_pass = hash_password(&payload.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to hash password: {}", e)})),
        )
    })?;
    let user = sqlx::query_as::<_, User>(
        r#"
    INSERT INTO users (id,name,email,username,password,created_at)
    VALUES ($1,$2,$3,$4,$5,NOW())
    RETURNING *
    "#,
    )
    .bind(Uuid::new_v4())
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.username)
    .bind(&hash_pass)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create user: {}", e)})),
        )
    })?;
    let token =
        create_token(&user.id.to_string(), &user.username, &state.jwt_secret).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create token: {}", e)})),
            )
        })?;
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: user.into(),
        }),
    ))
}
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    let user = query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;
    let user = user.ok_or((
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid credentials"})),
    ))?;
    let is_valid = verify_password(&payload.password, &user.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Password verification error: {}", e)})),
        )
    })?;
    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }
    let token =
        create_token(&user.id.to_string(), &user.username, &state.jwt_secret).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create token: {}", e)})),
            )
        })?;
    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}
