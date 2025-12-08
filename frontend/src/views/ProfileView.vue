<script setup lang="ts">
import { onMounted } from 'vue'
import { useAuth } from '@/composables/useAuth'

const { user, isLoading, logout, fetchUser } = useAuth()

// onMounted — выполняется когда компонент появляется на странице
onMounted(() => {
  fetchUser()
})
</script>

<template>
  <div class="profile-container">
    <h1>Профиль</h1>

    <div v-if="isLoading" class="loading">Загрузка...</div>

    <div v-else-if="user" class="profile-card">
      <div class="profile-item">
        <span class="label">Имя:</span>
        <span class="value">{{ user.first_name }} {{ user.last_name }}</span>
      </div>

      <div class="profile-item">
        <span class="label">Email:</span>
        <span class="value">{{ user.email }}</span>
      </div>

      <div class="profile-item">
        <span class="label">ID:</span>
        <span class="value">{{ user.id }}</span>
      </div>

      <div class="profile-item">
        <span class="label">Дата регистрации:</span>
        <span class="value">{{ new Date(user.created_at).toLocaleDateString('ru-RU') }}</span>
      </div>

      <button @click="logout" class="logout-btn">Выйти</button>
    </div>
  </div>
</template>

<style scoped>
.profile-container {
  max-width: 500px;
  margin: 50px auto;
  padding: 20px;
}

h1 {
  text-align: center;
  margin-bottom: 20px;
}

.loading {
  text-align: center;
  color: #666;
}

.profile-card {
  background: #f5f5f5;
  padding: 20px;
  border-radius: 8px;
}

.profile-item {
  margin-bottom: 15px;
  padding-bottom: 15px;
  border-bottom: 1px solid #ddd;
}

.profile-item:last-of-type {
  border-bottom: none;
}

.label {
  font-weight: bold;
  display: block;
  margin-bottom: 5px;
  color: #666;
}

.value {
  font-size: 18px;
}

.logout-btn {
  width: 100%;
  padding: 12px;
  background-color: #f44336;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  cursor: pointer;
  margin-top: 20px;
}

.logout-btn:hover {
  background-color: #d32f2f;
}
</style>