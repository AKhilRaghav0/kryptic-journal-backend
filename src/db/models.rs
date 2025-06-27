use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String, // This will be encrypted
    pub mood_score: Option<i32>, // 1-10 scale
    pub tags: Option<Vec<String>>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateJournalEntry {
    pub title: String,
    pub content: String,
    pub mood_score: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateJournalEntry {
    pub title: Option<String>,
    pub content: Option<String>,
    pub mood_score: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct JournalEntryResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String, // This will be decrypted before sending
    pub mood_score: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
} 