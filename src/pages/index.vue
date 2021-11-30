<script setup lang="ts">
import { useGameStore } from '~/stores/game'
const gameState = useGameStore()

</script>

<template>
  <div class="py-5">
    <Game />
  </div>
</template>

<route lang="yaml">
meta:
  layout: default
</route>
