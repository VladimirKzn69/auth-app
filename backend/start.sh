#!/bin/bash
set -e # Выйти немедленно при ошибке

echo "=== Запуск миграций ==="
# DATABASE_URL должен быть доступен как переменная окружения в Render
# Предполагается, что sqlx-cli установлен в билд-команде
# Мы находимся в папке backend/ благодаря Start Command
sqlx migrate run

if [ $? -eq 0 ]; then
    echo "=== Миграции успешно применены ==="
else
    echo "=== ОШИБКА: Миграции не удалось применить ==="
    exit 1
fi

echo "=== Запуск бэкенда ==="
# Запускаем бинарник из target/release/
# Имя бинарника берётся из Cargo.toml -> [package].name
# В вашем Cargo.toml: name = "backend"
exec ./target/release/backend