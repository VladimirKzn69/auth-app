use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ::std::time::Duration;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod models;
mod repositories;
mod services;
mod handlers;

#[derive(Clone)] // Важно для axum: состояние должно быть клонируемым
pub struct AppState {
    db: Pool<Postgres>, // Пул соединений с PostgreSQL
    jwt_secret: String, // Секретный ключ для JWT
}
async fn connect_db() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL") // Получаем URL из .env
        .expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5) // Ограничиваем количество соединений
        .acquire_timeout(Duration::from_secs(3)) // Таймаут ожидания соединения
        .connect(&database_url) // Подключаемся к БД
        .await
        .expect("Failed to create pool") // Критическая ошибка, если подключиться не удалось
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let db_pool = connect_db().await; // Создаём пул
    let jwt_secret = std::env::var("JWT_SECRET") // Получаем JWT_SECRET
        .expect("JWT_SECRET must be set");
    let app_state = AppState {
        // Создаём состояние
        db: db_pool,
        jwt_secret,
    };

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
    .route(
        "/",
        get(|| async { "✅ Clean auth-app backend is running!" }),
    )
    .route("/register", post(handlers::auth_handler::register))
    .route("/login", post(handlers::auth_handler::login))
    .layer(cors)
    .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 Server started on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
