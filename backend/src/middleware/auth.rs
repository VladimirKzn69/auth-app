use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use uuid::Uuid;

use crate::AppState;
use crate::services::jwt_service;

// ═══════════════════════════════════════════════════════════════════
// Auth Middleware — проверка JWT-токена
// ═══════════════════════════════════════════════════════════════════
// Этот middleware:
// 1. Извлекает токен из заголовка Authorization
// 2. Проверяет валидность токена
// 3. Добавляет user_id в расширения запроса
// 4. Пропускает запрос дальше или возвращает 401

/// Middleware для проверки JWT-токена
/// 
/// # Как использовать
/// Добавить к защищённым маршрутам через `.layer()`
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Шаг 1: Получаем заголовок Authorization
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    // Шаг 2: Проверяем формат "Bearer <token>"
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.trim_start_matches("Bearer ").to_string()
        }
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Missing or invalid authorization header" })),
            ));
        }
    };

    // Шаг 3: Проверяем токен
    let claims = match jwt_service::verify_token(&token, &state.jwt_secret) {
        Ok(claims) => claims,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid or expired token" })),
            ));
        }
    };

    // Шаг 4: Парсим user_id из claims
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid token payload" })),
            ));
        }
    };

    // Шаг 5: Добавляем user_id в расширения запроса
    // Это позволит handler'у получить ID текущего пользователя
    request.extensions_mut().insert(user_id);

    // Шаг 6: Пропускаем запрос дальше
    Ok(next.run(request).await)
}
