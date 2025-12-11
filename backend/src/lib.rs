// ═══════════════════════════════════════════════════════════════════
// lib.rs — точка входа библиотеки
// ═══════════════════════════════════════════════════════════════════
// Этот файл экспортирует модули для использования в тестах
// и потенциально в других проектах.

use axum::middleware as axum_middleware;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use http::Method;

// Публичные модули
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;

// ═══════════════════════════════════════════════════════════════════
// AppState — состояние приложения
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub jwt_secret: String,
}

// ═══════════════════════════════════════════════════════════════════
// Подключение к БД
// ═══════════════════════════════════════════════════════════════════

pub async fn connect_db(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
        .expect("Failed to create pool")
}

// ═══════════════════════════════════════════════════════════════════
// Применение миграций
// ═══════════════════════════════════════════════════════════════════

pub async fn run_migrations(pool: &Pool<Postgres>) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");
}

// ═══════════════════════════════════════════════════════════════════
// Создание приложения Axum
// ═══════════════════════════════════════════════════════════════════

pub fn create_app(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    // Защищённые маршруты (требуют JWT)
    let protected_routes = Router::new()
        .route("/me", get(handlers::auth_handler::me))
        .layer(axum_middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth::auth_middleware,
        ));

    // Основное приложение
    Router::new()
        .route("/", get(|| async { "✅ Clean auth-app backend is running!" }))
        .route("/register", post(handlers::auth_handler::register))
        .route("/login", post(handlers::auth_handler::login))
        .merge(protected_routes)
        .layer(cors)
        .with_state(app_state)
}