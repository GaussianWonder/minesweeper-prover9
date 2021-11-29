<script setup lang="ts">
import { isDark, toggleDark } from '~/composables'
</script>

<template>
  <nav class="text-xl flex flex-row items-center justify-center gap-4 py-2">
    <router-link class="icon-btn mx-2" to="/" title="Home">
      <carbon-template />
    </router-link>

    <button class="icon-btn mx-2 !outline-none" title="Toggle Dark Mode" @click="toggleDark()">
      <carbon-moon v-if="isDark" />
      <carbon-sun v-else />
    </button>

    <router-link class="icon-btn mx-2" to="/about" title="About">
      <carbon-dicom-overlay />
    </router-link>
  </nav>
</template>
