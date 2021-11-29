<script setup lang="ts">
// https://github.com/vueuse/head
// you can use this to manipulate the document head in any components,
// they will be rendered correctly in the html results with vite-ssg
useHead({
  title: 'Tauri-App',
  meta: [
    { name: 'description', content: 'Opinionated Vite-Tauri Starter Template' },
  ],
})
</script>

<template>
  <router-view />
</template>
