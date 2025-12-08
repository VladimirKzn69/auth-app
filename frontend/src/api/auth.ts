// ═══════════════════════════════════════════════════════════════════
// Auth API — функции для работы с эндпоинтами аутентификации
// ═══════════════════════════════════════════════════════════════════
// Аналог вызовов к handlers в Rust-бэкенде.

import apiClient from './client'

// Типы данных (как struct в Rust)
export interface RegisterData {
  email: string
  password: string
  first_name: string
  last_name: string
}

export interface LoginData {
  email: string
  password: string
}

export interface User {
  id: string
  email: string
  first_name: string
  last_name: string
  created_at: string
}

export interface LoginResponse {
  user: User
  token: string
}

// ═══════════════════════════════════════════════════════════════════
// API-функции
// ═══════════════════════════════════════════════════════════════════

// POST /register — регистрация нового пользователя
export async function register(data: RegisterData): Promise<User> {
  const response = await apiClient.post<User>('/register', data)
  return response.data
}

// POST /login — вход пользователя
export async function login(data: LoginData): Promise<LoginResponse> {
  const response = await apiClient.post<LoginResponse>('/login', data)
  return response.data
}

// GET /me — получить данные текущего пользователя
export async function getMe(): Promise<User> {
  const response = await apiClient.get<User>('/me')
  return response.data
}