<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '@/composables/useAuth'

const { login, isLoading, error } = useAuth()

const form = ref({
  email: '',
  password: '',
})

async function handleSubmit() {
  await login(form.value)
}
</script>

<template>
  <div class="auth-container">
    <h1>Вход</h1>

    <div v-if="error" class="error">{{ error }}</div>

    <form @submit.prevent="handleSubmit">
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
          placeholder="Введите пароль"
        />
      </div>

      <button type="submit" :disabled="isLoading">
        {{ isLoading ? 'Загрузка...' : 'Войти' }}
      </button>
    </form>

    <p class="link">
      Нет аккаунта? <router-link to="/register">Зарегистрироваться</router-link>
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
  background-color: #1976d2;
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
  background-color: #1565c0;
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