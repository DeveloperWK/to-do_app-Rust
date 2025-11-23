use crate::db::DBpool;
use axum::extract::FromRef;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub db: DBpool,
    pub jwt_secret: String,
}
impl FromRef<AppState> for JwtConfig {
    fn from_ref(app: &AppState) -> Self {
        JwtConfig {
            secret: app.jwt_secret.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    secret: String,
}
