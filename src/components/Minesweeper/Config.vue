<script setup lang="ts">
import AltButtonVue from '../ui/AltButton.vue'
import WarnButtonVue from '../ui/WarnButton.vue'

import { useGameStore } from '~/stores/game'
const gameState = useGameStore()

const rows = ref(9)
const cols = ref(9)
const bombs = ref(10)

const isConfigDirty = computed(() => {
  const { row, col, bomb } = gameState.config
  return row !== rows.value || col !== cols.value || bomb !== bombs.value
})

const resetButtonType = computed(() => {
  return isConfigDirty.value ? WarnButtonVue : AltButtonVue
})

const resetGame = () => {
  gameState.setRowCount(rows.value)
  gameState.setColCount(cols.value)
  gameState.setBombCount(bombs.value)
  // TODO reset logic
}
</script>

<template>
  <div class="flex flex-row gap-1">
    <div class="px-5 py-2 border border-gray-700 rounded-lg flex flex-row gap-2 items-center">
      <span>Grid </span>
      <div class="h-full w-px bg-black dark:bg-white opacity-10" />
      <PrimaryInput
        v-model.number="rows"
        type="number"
        class="w-22"
      >
        <template #label>
          Rows
        </template>
      </PrimaryInput>
      <span>x</span>
      <PrimaryInput
        v-model.number="cols"
        type="number"
        class="w-22"
      >
        <template #label>
          Cols
        </template>
      </PrimaryInput>
    </div>
    <div class="px-5 py-2 border border-gray-700 rounded-lg flex flex-row gap-2 items-center">
      <span>Bombs </span>
      <div class="h-full w-px bg-black dark:bg-white opacity-10" />
      <PrimaryInput
        v-model.number="bombs"
        type="number"
        class="w-22"
      >
        <template #label>
          Count
        </template>
      </PrimaryInput>
    </div>
    <div class="relative">
      <Transition name="component-fade">
        <component
          :is="resetButtonType"
          class="absolute px-5 py-2 rounded-lg"
          @click="resetGame"
        >
          Reset
        </component>
      </Transition>
    </div>
    <!-- <AltButton class="px-5 py-2 rounded-lg">
      Reset
    </AltButton> -->
  </div>
</template>

<style scoped>
.component-fade-enter-active,
.component-fade-leave-active {
  transition: opacity 0.3s ease;
}

.component-fade-enter-from,
.component-fade-leave-to {
  opacity: 0;
}
</style>
