// ═══════════════════════════════════════════════════════════════════
// useAuth — composable для управления аутентификацией
// ═══════════════════════════════════════════════════════════════════

import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import * as authApi from '@/api/auth'
import type { User, RegisterData, LoginData } from '@/api/auth'

// ═══════════════════════════════════════════════════════════════════
// Тип для ошибок Axios
// ═══════════════════════════════════════════════════════════════════
interface ApiError {
  response?: {
    data?: {
      error?: string
    }
  }
}

// ═══════════════════════════════════════════════════════════════════
// Глобальное состояние
// ═══════════════════════════════════════════════════════════════════
const user = ref<User | null>(null)
const token = ref<string | null>(localStorage.getItem('token'))
const isLoading = ref(false)
const error = ref<string | null>(null)

// ═══════════════════════════════════════════════════════════════════
// Composable-функция
// ═══════════════════════════════════════════════════════════════════
export function useAuth() {
  const router = useRouter()

  const isAuthenticated = computed(() => !!token.value)

  // ─────────────────────────────────────────────────────────────────
  // Регистрация нового пользователя
  // ─────────────────────────────────────────────────────────────────
  async function register(data: RegisterData): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      await authApi.register(data)
      router.push('/login')
      return true
    } catch (err: unknown) {
      const apiError = err as ApiError
      error.value = apiError.response?.data?.error || 'Ошибка регистрации'
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
      token.value = response.token
      user.value = response.user
      localStorage.setItem('token', response.token)
      router.push('/profile')
      return true
    } catch (err: unknown) {
      const apiError = err as ApiError
      error.value = apiError.response?.data?.error || 'Неверный email или пароль'
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
    } catch {
      logout()
    } finally {
      isLoading.value = false
    }
  }

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