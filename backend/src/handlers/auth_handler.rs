use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
// use sqlx::{Pool, Postgres};
use crate::AppState;

use crate::models::{CreateUser, UserResponse};
use crate::services::auth_service::{self, RegisterError};

// ═══════════════════════════════════════════════════════════════════
// AuthHandler — HTTP-обработчики для аутентификации
// ═══════════════════════════════════════════════════════════════════
// Этот слой отвечает ТОЛЬКО за:
// - Принять HTTP-запрос
// - Вызвать нужный service
// - Вернуть HTTP-ответ
//
// Никакой бизнес-логики здесь нет!

/// POST /register — регистрация нового пользователя
/// 
/// # Принимает
/// JSON с полями: email, password, first_name, last_name
/// 
/// # Возвращает
/// * 201 Created + данные пользователя (без пароля)
/// * 409 Conflict — email уже занят
/// * 500 Internal Server Error — ошибка сервера
pub async fn register(
    State(state): State<AppState>,
    Json(user_data): Json<CreateUser>,
) -> impl IntoResponse {
    // Вызываем service
    match auth_service::register_user(&state.db, user_data).await {
        // Успех — возвращаем 201 Created
        Ok(user) => {
            let response = UserResponse::from(user);
            (StatusCode::CREATED, Json(response)).into_response()
        }
        // Email занят — 409 Conflict
        Err(RegisterError::EmailAlreadyExists) => {
            (StatusCode::CONFLICT, Json(serde_json::json!({
                "error": "Email already exists"
            }))).into_response()
        }
        // Ошибка хеширования — 500
        Err(RegisterError::HashingError(msg)) => {
            tracing::error!("Password hashing error: {}", msg);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Internal server error"
            }))).into_response()
        }
        // Ошибка БД — 500
        Err(RegisterError::DatabaseError(e)) => {
            tracing::error!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Internal server error"
            }))).into_response()
        }
    }
}