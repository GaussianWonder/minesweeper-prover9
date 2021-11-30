<script setup lang="ts">
const props = defineProps({ digit: Number })
</script>

<template>
  <carbon-number-0 v-if="props.digit === 0" />
  <carbon-number-1 v-else-if="props.digit === 1" />
  <carbon-number-2 v-else-if="props.digit === 2" />
  <carbon-number-3 v-else-if="props.digit === 3" />
  <carbon-number-4 v-else-if="props.digit === 4" />
  <carbon-number-5 v-else-if="props.digit === 5" />
  <carbon-number-6 v-else-if="props.digit === 6" />
  <carbon-number-7 v-else-if="props.digit === 7" />
  <carbon-number-8 v-else-if="props.digit === 8" />
  <carbon-number-9 v-else-if="props.digit === 9" />
</template>
