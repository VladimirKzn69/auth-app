// ═══════════════════════════════════════════════════════════════════
// Integration-тесты для аутентификации
// ═══════════════════════════════════════════════════════════════════

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use serde_json::json;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use tower::ServiceExt;

// Импортируем из нашего приложения
use backend::{create_app, AppState};

// ═══════════════════════════════════════════════════════════════════
// Вспомогательные функции
// ═══════════════════════════════════════════════════════════════════

async fn get_test_db() -> Pool<Postgres> {
    // Берём URL из переменной окружения или используем дефолтный (локальный)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://dev_user:dev_pass@localhost:5435/auth_db_test".to_string());
    
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

async fn cleanup_db(pool: &Pool<Postgres>) {
    sqlx::query("DELETE FROM users")
        .execute(pool)
        .await
        .expect("Failed to cleanup users table");
}

fn create_test_state(pool: Pool<Postgres>) -> AppState {
    AppState {
        db: pool,
        jwt_secret: "test_secret_key_for_testing".to_string(),
    }
}

// ═══════════════════════════════════════════════════════════════════
// Тесты регистрации
// ═══════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_register_success() {
    let pool = get_test_db().await;
    cleanup_db(&pool).await;
    
    let app_state = create_test_state(pool);
    let app = create_app(app_state);

    let body = json!({
        "email": "test@example.com",
        "password": "TestPass123!",
        "first_name": "Test",
        "last_name": "User"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let pool = get_test_db().await;
    cleanup_db(&pool).await;
    
    let app_state = create_test_state(pool.clone());
    let app = create_app(app_state);

    let body = json!({
        "email": "duplicate@example.com",
        "password": "TestPass123!",
        "first_name": "Test",
        "last_name": "User"
    });

    // Первая регистрация — успех
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Вторая регистрация с тем же email — ошибка
    let app_state2 = create_test_state(pool);
    let app2 = create_app(app_state2);
    
    let response2 = app2
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_register_weak_password() {
    let pool = get_test_db().await;
    cleanup_db(&pool).await;
    
    let app_state = create_test_state(pool);
    let app = create_app(app_state);

    let body = json!({
        "email": "weak@example.com",
        "password": "short",
        "first_name": "Test",
        "last_name": "User"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ═══════════════════════════════════════════════════════════════════
// Тесты логина
// ═══════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_login_success() {
    let pool = get_test_db().await;
    cleanup_db(&pool).await;
    
    // Сначала регистрируем
    let app_state = create_test_state(pool.clone());
    let app = create_app(app_state);

    let register_body = json!({
        "email": "login@example.com",
        "password": "TestPass123!",
        "first_name": "Test",
        "last_name": "User"
    });

    app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/register")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(register_body.to_string()))
            .unwrap(),
    )
    .await
    .unwrap();

    // Теперь логинимся
    let app_state2 = create_test_state(pool);
    let app2 = create_app(app_state2);

    let login_body = json!({
        "email": "login@example.com",
        "password": "TestPass123!"
    });

    let response = app2
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_wrong_password() {
    let pool = get_test_db().await;
    cleanup_db(&pool).await;
    
    // Регистрируем
    let app_state = create_test_state(pool.clone());
    let app = create_app(app_state);

    let register_body = json!({
        "email": "wrongpass@example.com",
        "password": "TestPass123!",
        "first_name": "Test",
        "last_name": "User"
    });

    app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/register")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(register_body.to_string()))
            .unwrap(),
    )
    .await
    .unwrap();

    // Логин с неверным паролем
    let app_state2 = create_test_state(pool);
    let app2 = create_app(app_state2);

    let login_body = json!({
        "email": "wrongpass@example.com",
        "password": "WrongPassword123!"
    });

    let response = app2
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}