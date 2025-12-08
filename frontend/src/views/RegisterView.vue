<!-- ═══════════════════════════════════════════════════════════════════
     RegisterView — страница регистрации
     ═══════════════════════════════════════════════════════════════════
     Компонент Vue состоит из трёх частей:
     - <script setup> — логика (как Rust-код)
     - <template> — разметка (как HTML)
     - <style> — стили (CSS)
-->

<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '@/composables/useAuth'

// Получаем методы и состояние из composable
const { register, isLoading, error } = useAuth()

// Локальное состояние формы (ref — реактивная переменная)
const form = ref({
  email: '',
  password: '',
  first_name: '',
  last_name: '',
})

// Обработчик отправки формы
async function handleSubmit() {
  await register(form.value)
}
</script>

<template>
  <div class="auth-container">
    <h1>Регистрация</h1>

    <!-- Показываем ошибку, если есть -->
    <div v-if="error" class="error">{{ error }}</div>

    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="first_name">Имя</label>
        <input
          id="first_name"
          v-model="form.first_name"
          type="text"
          required
          placeholder="Введите имя"
        />
      </div>

      <div class="form-group">
        <label for="last_name">Фамилия</label>
        <input
          id="last_name"
          v-model="form.last_name"
          type="text"
          required
          placeholder="Введите фамилию"
        />
      </div>

      <div class="form-group">
        <label for="email">Email</label>
        <input
          id="email"
          v-model="form.email"
          type="email"
          required
          placeholder="example@mail.com"
        />
      </div>

      <div class="form-group">
        <label for="password">Пароль</label>
        <input
          id="password"
          v-model="form.password"
          type="password"
          required
          placeholder="Минимум 8 символов, A-Z, a-z, 0-9, спецсимвол"
        />
      </div>

      <button type="submit" :disabled="isLoading">
        {{ isLoading ? 'Загрузка...' : 'Зарегистрироваться' }}
      </button>
    </form>

    <p class="link">
      Уже есть аккаунт? <router-link to="/login">Войти</router-link>
    </p>
  </div>
</template>

<style scoped>
.auth-container {
  max-width: 400px;
  margin: 50px auto;
  padding: 20px;
}

h1 {
  text-align: center;
  margin-bottom: 20px;
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 16px;
}

button {
  width: 100%;
  padding: 12px;
  background-color: #4caf50;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  cursor: pointer;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

button:hover:not(:disabled) {
  background-color: #45a049;
}

.error {
  background-color: #ffebee;
  color: #c62828;
  padding: 10px;
  border-radius: 4px;
  margin-bottom: 15px;
}

.link {
  text-align: center;
  margin-top: 15px;
}

.link a {
  color: #1976d2;
}
</style>