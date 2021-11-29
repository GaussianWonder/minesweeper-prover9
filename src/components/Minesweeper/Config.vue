<script setup lang="ts">
const rows = ref(9)
const cols = ref(9)
const bombs = ref(10)
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
    <AltButton class="px-5 py-2 rounded-lg">
      Reset
    </AltButton>
  </div>
</template>
