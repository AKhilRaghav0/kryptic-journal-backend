use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::models::{CreateJournalEntry, JournalEntry, JournalEntryResponse, UpdateJournalEntry};
use crate::utils::encryption::{encrypt_text, decrypt_text};
use crate::AppState;

pub async fn create_entry(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Json(payload): Json<CreateJournalEntry>,
) -> Result<Json<JournalEntryResponse>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Encrypt content
    let encrypted_content = encrypt_text(&payload.content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let entry_id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();

    let entry = sqlx::query_as::<_, JournalEntry>(
        r#"
        INSERT INTO journal_entries (id, user_id, title, content, mood_score, tags, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, title, content, mood_score, tags, created_at, updated_at
        "#
    )
    .bind(entry_id)
    .bind(user_uuid)
    .bind(&payload.title)
    .bind(&encrypted_content)
    .bind(payload.mood_score)
    .bind(&payload.tags)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Decrypt content for response
    let decrypted_content = decrypt_text(&entry.content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(JournalEntryResponse {
        id: entry.id,
        title: entry.title,
        content: decrypted_content,
        mood_score: entry.mood_score,
        tags: entry.tags,
        created_at: entry.created_at,
        updated_at: entry.updated_at,
    }))
}

pub async fn get_entries(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
) -> Result<Json<Vec<JournalEntryResponse>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let entries = sqlx::query_as::<_, JournalEntry>(
        "SELECT id, user_id, title, content, mood_score, tags, created_at, updated_at FROM journal_entries WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_uuid)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut response_entries = Vec::new();
    for entry in entries {
        let decrypted_content = decrypt_text(&entry.content)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        response_entries.push(JournalEntryResponse {
            id: entry.id,
            title: entry.title,
            content: decrypted_content,
            mood_score: entry.mood_score,
            tags: entry.tags,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        });
    }

    Ok(Json(response_entries))
}

pub async fn get_entry(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(entry_id): Path<Uuid>,
) -> Result<Json<JournalEntryResponse>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let entry = sqlx::query_as::<_, JournalEntry>(
        "SELECT id, user_id, title, content, mood_score, tags, created_at, updated_at FROM journal_entries WHERE id = $1 AND user_id = $2"
    )
    .bind(entry_id)
    .bind(user_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Decrypt content for response
    let decrypted_content = decrypt_text(&entry.content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(JournalEntryResponse {
        id: entry.id,
        title: entry.title,
        content: decrypted_content,
        mood_score: entry.mood_score,
        tags: entry.tags,
        created_at: entry.created_at,
        updated_at: entry.updated_at,
    }))
}

pub async fn update_entry(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(entry_id): Path<Uuid>,
    Json(payload): Json<UpdateJournalEntry>,
) -> Result<Json<JournalEntryResponse>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // First check if entry exists and belongs to user
    let existing_entry = sqlx::query_as::<_, JournalEntry>(
        "SELECT id, user_id, title, content, mood_score, tags, created_at, updated_at FROM journal_entries WHERE id = $1 AND user_id = $2"
    )
    .bind(entry_id)
    .bind(user_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Prepare update fields
    let title = payload.title.as_ref().unwrap_or(&existing_entry.title);
    
    let encrypted_content = if let Some(content) = &payload.content {
        encrypt_text(content)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        existing_entry.content.clone()
    };

    let mood_score = payload.mood_score.or(existing_entry.mood_score);
    let tags = payload.tags.as_ref().or(existing_entry.tags.as_ref());
    let now = OffsetDateTime::now_utc();

    let updated_entry = sqlx::query_as::<_, JournalEntry>(
        r#"
        UPDATE journal_entries 
        SET title = $1, content = $2, mood_score = $3, tags = $4, updated_at = $5
        WHERE id = $6 AND user_id = $7
        RETURNING id, user_id, title, content, mood_score, tags, created_at, updated_at
        "#
    )
    .bind(title)
    .bind(&encrypted_content)
    .bind(mood_score)
    .bind(tags)
    .bind(now)
    .bind(entry_id)
    .bind(user_uuid)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Decrypt content for response
    let decrypted_content = decrypt_text(&updated_entry.content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(JournalEntryResponse {
        id: updated_entry.id,
        title: updated_entry.title,
        content: decrypted_content,
        mood_score: updated_entry.mood_score,
        tags: updated_entry.tags,
        created_at: updated_entry.created_at,
        updated_at: updated_entry.updated_at,
    }))
}

pub async fn delete_entry(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(entry_id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = sqlx::query(
        "DELETE FROM journal_entries WHERE id = $1 AND user_id = $2"
    )
    .bind(entry_id)
    .bind(user_uuid)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "message": "Entry deleted successfully"
    })))
} 