#!/bin/bash
set -e # Выйти немедленно при ошибке

echo "=== Запуск миграций ==="
# DATABASE_URL должен быть доступен как переменная окружения в Render
# Проверим, установлена ли она
if [ -z "$DATABASE_URL" ]; then
  echo "❌ ОШИБКА: Переменная DATABASE_URL не установлена!"
  exit 1
fi

echo "DATABASE_URL обнаружен (последние 10 символов): ...${DATABASE_URL: -10}"

# Предполагается, что sqlx-cli установлен в билд-команде
sqlx migrate run

if [ $? -eq 0 ]; then
    echo "=== Миграции успешно применены ==="
else
    echo "=== ОШИБКА: Миграции не удалось применить ==="
    exit 1
fi

echo "=== Запуск бэкенда ==="
# Запускаем бинарник из target/release/
exec ./target/release/backend
