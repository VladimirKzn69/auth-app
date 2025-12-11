// ═══════════════════════════════════════════════════════════════════
// main.rs — точка входа приложения
// ═══════════════════════════════════════════════════════════════════

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Импортируем из нашей библиотеки
use backend::{connect_db, create_app, AppState};

#[tokio::main]
async fn main() {
    // Инициализация логирования
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Загрузка переменных окружения
    dotenvy::dotenv().ok();

    // Подключение к БД
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db_pool = connect_db(&database_url).await;

    // 🆕 Применение миграций
    backend::run_migrations(&db_pool).await;
    tracing::info!("✅ Migrations applied successfully");
    
    // Получение JWT-секрета
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    // Создание состояния приложения
    let app_state = AppState {
        db: db_pool,
        jwt_secret,
    };

    // Создание приложения
    let app = create_app(app_state);

    // Запуск сервера
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 Server started on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}