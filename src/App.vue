<script setup lang="ts">
// https://github.com/vueuse/head
// you can use this to manipulate the document head in any components,
// they will be rendered correctly in the html results with vite-ssg
useHead({
  title: 'Minesweeper Prover9',
  meta: [
    { name: 'description', content: 'Minesweeper puzzle solvable with Prover9' },
  ],
})
</script>

<template>
  <router-view />
</template>
