<script setup lang="ts">
import AltButtonVue from '~/components/ui/AltButton.vue'
import WarnButtonVue from '~/components/ui/WarnButton.vue'

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

const validRows = computed(() => {
  return rows.value.toString() !== '' && rows.value > 0 && rows.value <= 12
})

const validCols = computed(() => {
  return cols.value.toString() !== '' && cols.value > 0 && cols.value <= 20
})

const validBombs = computed(() => {
  const totalFields = rows.value * cols.value
  if (totalFields <= 0) return false

  const bombRatio = bombs.value / totalFields
  return bombs.value.toString() !== '' && bombs.value > 0 && bombRatio < 0.5
})

const resetGame = () => {
  gameState.setRowCount(rows.value)
  gameState.setColCount(cols.value)
  gameState.setBombCount(bombs.value)
  gameState.setIsFreshState(true)
}
</script>

<template>
  <div class="flex flex-row gap-1">
    <div class="px-5 py-2 border border-gray-700 rounded-lg flex flex-row gap-2 items-center">
      <span>Grid </span>
      <div class="h-full w-px bg-black dark:bg-white opacity-10" />
      <GuardedInput
        v-model.number="rows"
        :danger="!validRows"
        type="number"
        class="w-22"
      >
        <template #label>
          Rows
        </template>
      </GuardedInput>
      <span>x</span>
      <GuardedInput
        v-model.number="cols"
        :danger="!validCols"
        type="number"
        class="w-22"
      >
        <template #label>
          Cols
        </template>
      </GuardedInput>
    </div>
    <div class="px-5 py-2 border border-gray-700 rounded-lg flex flex-row gap-2 items-center">
      <span>Bombs </span>
      <div class="h-full w-px bg-black dark:bg-white opacity-10" />
      <GuardedInput
        v-model.number="bombs"
        :danger="!validBombs"
        type="number"
        class="w-22"
      >
        <template #label>
          Count
        </template>
      </GuardedInput>
    </div>
    <div class="relative">
      <Transition name="component-fade">
        <component
          :is="resetButtonType"
          class="absolute px-5 py-2 rounded-lg"
          :disabled="!validRows || !validCols || !validBombs"
          @click="resetGame"
        >
          Reset
        </component>
      </Transition>
    </div>
  </div>
</template>

<style scoped>
.component-fade-enter-active,
.component-fade-leave-active {
  transition: opacity 0.27s cubic-bezier(0.4, 0, 0.2, 1);
}

.component-fade-enter-from,
.component-fade-leave-to {
  opacity: 0;
}
</style>
