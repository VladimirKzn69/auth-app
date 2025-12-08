use sqlx::{Pool, Postgres};

use crate::models::{CreateUser, User};

// ═══════════════════════════════════════════════════════════════════
// UserRepository — слой работы с таблицей users в БД
// ═══════════════════════════════════════════════════════════════════
// Этот слой отвечает ТОЛЬКО за SQL-запросы.
// Никакой бизнес-логики (хеширование паролей, валидация) — только БД.

/// Создаёт нового пользователя в базе данных
///
/// # Аргументы
/// * `pool` — пул соединений с PostgreSQL
/// * `user` — данные для создания пользователя
/// * `password_hash` — уже захешированный пароль (хешируем в service, не здесь!)
///
/// # Возвращает
/// * `Ok(User)` — созданный пользователь с id и created_at
/// * `Err(sqlx::Error)` — ошибка БД (например, email уже существует)
pub async fn create_user(
    pool: &Pool<Postgres>,
    user: &CreateUser,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    // sqlx::query_as! — макрос, который:
    // 1. Проверяет SQL на этапе компиляции (!)
    // 2. Автоматически маппит результат в структуру User
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, first_name, last_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, password_hash, first_name, last_name, created_at
        "#,
        user.email,
        password_hash,
        user.first_name,
        user.last_name
    )
    .fetch_one(pool)
    .await
}

/// Ищет пользователя по email
///
/// # Аргументы
/// * `pool` — пул соединений с PostgreSQL
/// * `email` — email для поиска
///
/// # Возвращает
/// * `Ok(Some(User))` — пользователь найден
/// * `Ok(None)` — пользователь не найден
/// * `Err(sqlx::Error)` — ошибка БД
pub async fn find_by_email(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    // query_as! + fetch_optional = вернуть Option<User>
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, first_name, last_name, created_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
}

/// Ищет пользователя по ID
///
/// # Аргументы
/// * `pool` — пул соединений с PostgreSQL
/// * `id` — UUID пользователя
///
/// # Возвращает
/// * `Ok(Some(User))` — пользователь найден
/// * `Ok(None)` — пользователь не найден
/// * `Err(sqlx::Error)` — ошибка БД
pub async fn find_by_id(
    pool: &Pool<Postgres>,
    id: uuid::Uuid,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, first_name, last_name, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
