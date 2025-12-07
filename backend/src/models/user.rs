use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════
// User — полная структура из базы данных
// ═══════════════════════════════════════════════════════════════════
// Используется когда ЧИТАЕМ пользователя из БД.
// Содержит ВСЕ поля, включая password_hash.
//
// #[derive(...)] — это макросы, которые автоматически добавляют функциональность:
// - FromRow: позволяет SQLx преобразовывать строку из БД в эту структуру
// - Debug: позволяет печатать структуру для отладки
// - Clone: позволяет копировать структуру

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════
// CreateUser — данные для регистрации
// ═══════════════════════════════════════════════════════════════════
// Используется когда пользователь ОТПРАВЛЯЕТ форму регистрации.
// Обрати внимание: здесь password (не hash!), и нет id/created_at.
//
// - Deserialize: позволяет преобразовать JSON из запроса в структуру

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

// ═══════════════════════════════════════════════════════════════════
// LoginRequest — данные для входа
// ═══════════════════════════════════════════════════════════════════
// Используется когда пользователь ЛОГИНИТСЯ.
// Только email и password — больше ничего не нужно.

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// ═══════════════════════════════════════════════════════════════════
// UserResponse — данные для ответа клиенту
// ═══════════════════════════════════════════════════════════════════
// Используется когда ОТПРАВЛЯЕМ данные пользователю.
// БЕЗ password_hash — никогда не отдаём пароль наружу!
//
// - Serialize: позволяет преобразовать структуру в JSON для ответа

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════
// Преобразование User → UserResponse
// ═══════════════════════════════════════════════════════════════════
// Это impl (implementation) — добавляем методы к структуре.
// From<User> означает: "можно создать UserResponse из User"
//
// Зачем: когда читаем из БД, получаем User (с паролем).
// Чтобы отправить клиенту, конвертируем в UserResponse (без пароля).

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            created_at: user.created_at,
        }
    }
}