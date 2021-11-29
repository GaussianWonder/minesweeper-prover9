<script setup lang="ts">
</script>

<template>
  <BaseInput
    class="
    text-indigo-700 focus:(outline-none ring ring-indigo-100) bg-white border-indigo-200 hover:bg-indigo-50 active:bg-indigo-100
      dark:(text-indigo-200 focus:ring-indigo-800 bg-indigo-700 border-indigo-600 hover:bg-indigo-800 active-bg-indigo-900)
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
