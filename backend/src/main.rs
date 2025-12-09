use ::std::time::Duration;
use axum::middleware as axum_middleware;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod middleware;
mod models;
mod repositories;
mod services;

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
    let app = Router::new()
        .route(
            "/",
            get(|| async { "✅ Clean auth-app backend is running!" }),
        )
        .route("/register", post(handlers::auth_handler::register))
        .route("/login", post(handlers::auth_handler::login))
        .merge(protected_routes)
        .layer(cors)
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 Server started on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
