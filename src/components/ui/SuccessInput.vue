<script setup lang="ts">
</script>

<template>
  <BaseInput
    class="
    text-green-700 focus:(outline-none ring ring-green-100) bg-white border-green-200 hover:bg-green-50 active:bg-green-100
      dark:(text-green-200 focus:ring-green-800 bg-green-700 border-green-600 hover:bg-green-800 active-bg-green-900)
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
