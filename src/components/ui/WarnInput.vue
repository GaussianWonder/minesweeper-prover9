<script setup lang="ts">
</script>

<template>
  <BaseInput
    class="
    text-yellow-700 focus:(outline-none ring ring-yellow-100) bg-white border-yellow-200 hover:bg-yellow-50 active:bg-yellow-100
      dark:(text-yellow-200 focus:ring-yellow-800 bg-yellow-700 border-yellow-600 hover:bg-yellow-800 active-bg-yellow-900)
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
