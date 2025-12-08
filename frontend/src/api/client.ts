// ═══════════════════════════════════════════════════════════════════
// API Client — настройка Axios для работы с бэкендом
// ═══════════════════════════════════════════════════════════════════
// Этот файл — аналог repository в Rust.
// Здесь настраиваем базовый URL и перехватчики запросов/ответов.

import axios from 'axios'

// Создаём экземпляр axios с базовыми настройками
const apiClient = axios.create({
  baseURL: 'http://localhost:8080', // Адрес нашего Rust-бэкенда
  headers: {
    'Content-Type': 'application/json',
  },
})

// ═══════════════════════════════════════════════════════════════════
// Interceptor для запросов — добавляет токен к каждому запросу
// ═══════════════════════════════════════════════════════════════════
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  },
)

// ═══════════════════════════════════════════════════════════════════
// Interceptor для ответов — обрабатывает ошибки 401 (Unauthorized)
// ═══════════════════════════════════════════════════════════════════
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Токен истёк или невалидный — удаляем и редиректим на логин
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  },
)

export default apiClient