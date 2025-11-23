use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    Pending,
    Completed,
    Outdated,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todos {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub reminder: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub user_id: Uuid,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateTodos {
    pub title: String,
    pub description: String,
    pub status: String,
    pub reminder: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    // pub created_at: NaiveDateTime,
    pub user_id: Uuid,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateTodos {
    // pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub reminder: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
}
