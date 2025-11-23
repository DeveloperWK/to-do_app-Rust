use axum::{Extension, Json, http::StatusCode};
use serde_json::{Value, json};

use crate::models::auth::Claims;

pub async fn get_profile(
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Ok(Json(json!({
        "user_id": claims.sub,
        "username": claims.username,
        "message": "This is a protected route"
    })))
}
