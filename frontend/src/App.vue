<!-- ═══════════════════════════════════════════════════════════════════
     App.vue — корневой компонент приложения
     ═══════════════════════════════════════════════════════════════════
     Как main.rs — точка входа для всего UI.
-->

<script setup lang="ts">
// Здесь пока ничего не нужно
</script>

<template>
  <!-- RouterView — сюда подставляются страницы в зависимости от URL -->
  <RouterView />
</template>

<style>
/* Глобальные стили */
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
  line-height: 1.6;
  color: #333;
}

a {
  text-decoration: none;
}
</style>