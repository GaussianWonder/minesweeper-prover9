<script lang="ts">
import AltInputVue from '../ui/AltInput.vue'
import DangerInputVue from '../ui/DangerInput.vue'
import PrimaryInputVue from '../ui/PrimaryInput.vue'
import SuccessInputVue from '../ui/SuccessInput.vue'
import WarnInputVue from '../ui/WarnInput.vue'

export default {
  inheritAttrs: false,
}
</script>

<script setup lang="ts">
interface Props {
  danger?: () => boolean
  warn?: () => boolean
  success?: () => boolean
  primary?: () => boolean
  alt?: () => boolean
}

const props = withDefaults(defineProps<Props>(), {
  danger: () => false,
  warn: () => false,
  success: () => false,
  primary: () => false,
  alt: () => false,
})

const inputType = computed(() => {
  const [danger, warn, success, primary, alt] = [props.danger(), props.warn(), props.success(), props.primary(), props.alt()]
  if (danger) // priority line
    return DangerInputVue
  else if (warn)
    return WarnInputVue
  else if (success)
    return SuccessInputVue
  else if (primary)
    return PrimaryInputVue
  else if (alt)
    return AltInputVue
  return PrimaryInputVue // default
})
</script>

<template>
  <Transition name="component-fade" mode="out-in">
    <component
      :is="inputType"
      v-bind="$attrs"
    >
      <!-- Take all scoped slots, and pass them to the BaseInput -->
      <template
        v-for="(_, slot) in $slots"
        #[slot]="scope"
      >
        <slot :name="slot" v-bind="scope || {}" />
      </template>
    </component>
  </Transition>
</template>

<style scoped>
.component-fade-enter-active,
.component-fade-leave-active {
  transition: opacity 0.17s cubic-bezier(0.4, 0, 0.2, 1);
}

.component-fade-enter-from,
.component-fade-leave-to {
  opacity: 0;
}
</style>
