// ═══════════════════════════════════════════════════════════════════
// Router — навигация между страницами
// ═══════════════════════════════════════════════════════════════════
// Аналог маршрутов в Axum. Определяем какой URL → какой компонент.

import { createRouter, createWebHistory } from 'vue-router'

// Ленивая загрузка компонентов (загружаются только когда нужны)
const LoginView = () => import('@/views/LoginView.vue')
const RegisterView = () => import('@/views/RegisterView.vue')
const ProfileView = () => import('@/views/ProfileView.vue')

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/login', // Главная → перенаправляем на логин
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView,
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView,
    },
    {
      path: '/profile',
      name: 'profile',
      component: ProfileView,
      meta: { requiresAuth: true }, // Флаг: требуется авторизация
    },
  ],
})

// ═══════════════════════════════════════════════════════════════════
// Navigation Guard — защита маршрутов
// ═══════════════════════════════════════════════════════════════════
// Выполняется ПЕРЕД каждым переходом. Аналог middleware в Axum.

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('token')
  const isAuthenticated = !!token

  // Если маршрут требует авторизации и пользователь не залогинен
  if (to.meta.requiresAuth && !isAuthenticated) {
    next('/login') // Перенаправляем на логин
  }
  // Если залогиненный пользователь идёт на логин/регистрацию
  else if ((to.path === '/login' || to.path === '/register') && isAuthenticated) {
    next('/profile') // Перенаправляем на профиль
  }
  // Иначе — пропускаем
  else {
    next()
  }
})

export default router