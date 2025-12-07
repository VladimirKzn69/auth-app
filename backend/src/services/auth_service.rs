use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::{Pool, Postgres};

use crate::models::{CreateUser, User};
use crate::repositories::user_repository;

// ═══════════════════════════════════════════════════════════════════
// AuthService — бизнес-логика аутентификации
// ═══════════════════════════════════════════════════════════════════
// Этот слой отвечает за:
// - Хеширование паролей
// - Проверку уникальности email
// - Координацию между handler и repository

/// Ошибки, которые могут возникнуть при регистрации
#[derive(Debug)]
pub enum RegisterError {
    EmailAlreadyExists,
    HashingError(String),
    DatabaseError(sqlx::Error),
}

/// Регистрирует нового пользователя
/// 
/// # Шаги
/// 1. Проверяет, не занят ли email
/// 2. Хеширует пароль через argon2
/// 3. Сохраняет пользователя в БД
/// 
/// # Возвращает
/// * `Ok(User)` — успешно зарегистрированный пользователь
/// * `Err(RegisterError)` — ошибка регистрации
pub async fn register_user(
    pool: &Pool<Postgres>,
    user_data: CreateUser,
) -> Result<User, RegisterError> {
    // Шаг 1: Проверяем, не занят ли email
    let existing_user = user_repository::find_by_email(pool, &user_data.email)
        .await
        .map_err(RegisterError::DatabaseError)?;
    
    if existing_user.is_some() {
        return Err(RegisterError::EmailAlreadyExists);
    }

    // Шаг 2: Хешируем пароль
    let password_hash = hash_password(&user_data.password)
        .map_err(|e| RegisterError::HashingError(e.to_string()))?;

    // Шаг 3: Сохраняем пользователя в БД
    let user = user_repository::create_user(pool, &user_data, &password_hash)
        .await
        .map_err(RegisterError::DatabaseError)?;

    Ok(user)
}

/// Хеширует пароль с помощью Argon2
/// 
/// Argon2 автоматически генерирует уникальную соль для каждого хеша
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
/// Ошибки, которые могут возникнуть при логине
#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,      // Неверный email или пароль (одна ошибка для безопасности!)
    DatabaseError(sqlx::Error),
}

/// Авторизует пользователя по email и паролю
/// 
/// # Шаги
/// 1. Ищет пользователя по email
/// 2. Проверяет пароль через argon2
/// 
/// # Возвращает
/// * `Ok(User)` — успешная авторизация
/// * `Err(LoginError)` — ошибка авторизации
pub async fn login_user(
    pool: &Pool<Postgres>,
    email: &str,
    password: &str,
) -> Result<User, LoginError> {
    // Шаг 1: Ищем пользователя по email
    let user = user_repository::find_by_email(pool, email)
        .await
        .map_err(LoginError::DatabaseError)?
        .ok_or(LoginError::InvalidCredentials)?;  // Не найден = неверные данные

    // Шаг 2: Проверяем пароль
    if !verify_password(password, &user.password_hash) {
        return Err(LoginError::InvalidCredentials);  // Не совпал = неверные данные
    }

    Ok(user)
}

/// Проверяет пароль против хеша
/// 
/// Возвращает true если пароль верный, false если нет
pub fn verify_password(password: &str, hash: &str) -> bool {
    // Парсим хеш из строки
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    
    // Проверяем пароль
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}