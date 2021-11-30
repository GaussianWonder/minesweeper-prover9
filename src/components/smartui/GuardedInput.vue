<script setup lang="ts">
import { styles } from './button-styles'

interface Props {
  danger?: boolean
  warn?: boolean
  success?: boolean
  primary?: boolean
  alt?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  danger: false,
  warn: false,
  success: false,
  primary: false,
  alt: false,
})

const inputStyle = computed(() => {
  const { danger, warn, success, primary, alt } = props
  if (danger) // priority line
    return styles.danger
  else if (warn)
    return styles.warn
  else if (success)
    return styles.success
  else if (primary)
    return styles.primary
  else if (alt)
    return styles.alt
  return styles.primary // default
})
</script>

<template>
  <BaseInput :class="inputStyle">
    <!-- Take all scoped slots, and pass them to the BaseInput -->
    <template
      v-for="(_, slot) in $slots"
      #[slot]="scope"
    >
      <slot :name="slot" v-bind="scope || {}" />
    </template>
  </BaseInput>
</template>
