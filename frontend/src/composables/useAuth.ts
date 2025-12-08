// ═══════════════════════════════════════════════════════════════════
// useAuth — composable для управления аутентификацией
// ═══════════════════════════════════════════════════════════════════
// Аналог auth_service в Rust.
// Содержит состояние пользователя и методы для входа/выхода/регистрации.

import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import * as authApi from '@/api/auth'
import type { User, RegisterData, LoginData } from '@/api/auth'

// ═══════════════════════════════════════════════════════════════════
// Глобальное состояние (хранится между вызовами useAuth)
// ═══════════════════════════════════════════════════════════════════
// ref() — реактивная переменная. Когда она меняется, UI обновляется.
const user = ref<User | null>(null)
const token = ref<string | null>(localStorage.getItem('token'))
const isLoading = ref(false)
const error = ref<string | null>(null)

// ═══════════════════════════════════════════════════════════════════
// Composable-функция
// ═══════════════════════════════════════════════════════════════════
export function useAuth() {
  const router = useRouter()

  // computed() — вычисляемое свойство, автоматически пересчитывается
  const isAuthenticated = computed(() => !!token.value)

  // ─────────────────────────────────────────────────────────────────
  // Регистрация нового пользователя
  // ─────────────────────────────────────────────────────────────────
  async function register(data: RegisterData): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      await authApi.register(data)
      // После успешной регистрации перенаправляем на логин
      router.push('/login')
      return true
    } catch (err: any) {
      // Извлекаем сообщение об ошибке от бэкенда
      error.value = err.response?.data?.error || 'Ошибка регистрации'
      return false
    } finally {
      isLoading.value = false
    }
  }

  // ─────────────────────────────────────────────────────────────────
  // Вход пользователя
  // ─────────────────────────────────────────────────────────────────
  async function login(data: LoginData): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      const response = await authApi.login(data)
      // Сохраняем токен и данные пользователя
      token.value = response.token
      user.value = response.user
      localStorage.setItem('token', response.token)
      // Перенаправляем на профиль
      router.push('/profile')
      return true
    } catch (err: any) {
      error.value = err.response?.data?.error || 'Неверный email или пароль'
      return false
    } finally {
      isLoading.value = false
    }
  }

  // ─────────────────────────────────────────────────────────────────
  // Выход пользователя
  // ─────────────────────────────────────────────────────────────────
  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
    router.push('/login')
  }

  // ─────────────────────────────────────────────────────────────────
  // Загрузка данных текущего пользователя
  // ─────────────────────────────────────────────────────────────────
  async function fetchUser(): Promise<void> {
    if (!token.value) return

    isLoading.value = true
    try {
      user.value = await authApi.getMe()
    } catch (err: any) {
      // Если токен невалидный — выходим
      logout()
    } finally {
      isLoading.value = false
    }
  }

  // Возвращаем состояние и методы для использования в компонентах
  return {
    user,
    token,
    isLoading,
    error,
    isAuthenticated,
    register,
    login,
    logout,
    fetchUser,
  }
}