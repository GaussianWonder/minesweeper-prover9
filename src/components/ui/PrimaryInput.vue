<script setup lang="ts">
</script>

<template>
  <BaseInput
    class="
    text-gray-700 focus:(outline-none ring ring-gray-100) bg-white border-gray-200 hover:bg-gray-50 active:bg-gray-100
      dark:(text-gray-200 focus:ring-gray-800 bg-gray-700 border-gray-600 hover:bg-gray-800 active-bg-gray-900)
    "
  >
    <!-- Take all scoped slots, and pass them to the BaseInput -->
    <template
      v-for="(_, slot) in $slots"
      #[slot]="scope"
    >
      <slot :name="slot" v-bind="scope || {}" />
    </template>
  </BaseInput>
</template>
