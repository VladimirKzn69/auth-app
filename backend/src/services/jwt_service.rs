use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════
// JWT Service — работа с токенами
// ═══════════════════════════════════════════════════════════════════
// Этот модуль отвечает за:
// - Генерацию JWT-токенов при логине
// - Проверку и декодирование токенов

/// Claims — данные, которые хранятся внутри токена
///
/// # Поля
/// * `sub` — subject, ID пользователя (стандартное поле JWT)
/// * `exp` — expiration, время истечения токена (Unix timestamp)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id в виде строки
    pub exp: usize,  // время истечения (секунды с 1970-01-01)
}

/// Ошибки при работе с JWT
#[derive(Debug)]
pub enum JwtError {
    CreationError(String),
    ValidationError(String),
}

/// Генерирует JWT-токен для пользователя
///
/// # Аргументы
/// * `user_id` — UUID пользователя
/// * `secret` — секретный ключ для подписи
///
/// # Возвращает
/// * `Ok(String)` — готовый токен
/// * `Err(JwtError)` — ошибка генерации
pub fn create_token(user_id: Uuid, secret: &str) -> Result<String, JwtError> {
    // Токен живёт 1 час (3600 секунд)
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| JwtError::CreationError(e.to_string()))
}

/// Проверяет JWT-токен и возвращает данные из него
///
/// # Аргументы
/// * `token` — токен для проверки
/// * `secret` — секретный ключ для проверки подписи
///
/// # Возвращает
/// * `Ok(Claims)` — данные из токена
/// * `Err(JwtError)` — токен невалидный или истёк
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| JwtError::ValidationError(e.to_string()))
}
