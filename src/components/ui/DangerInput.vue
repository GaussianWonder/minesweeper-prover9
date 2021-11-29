<script setup lang="ts">
</script>

<template>
  <BaseInput
    class="
    text-red-700 focus:(outline-none ring ring-red-100) bg-white border-red-200 hover:bg-red-50 active:bg-red-100
      dark:(text-red-200 focus:ring-red-800 bg-red-700 border-red-600 hover:bg-red-800 active-bg-red-900)
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
