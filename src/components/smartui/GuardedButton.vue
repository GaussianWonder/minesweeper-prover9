<script lang="ts">
import DangerButtonVue from '~/components/ui/DangerButton.vue'
import WarnButtonVue from '~/components/ui/WarnButton.vue'
import SuccessButtonVue from '~/components/ui/SuccessButton.vue'
import PrimaryButtonVue from '~/components/ui/PrimaryButton.vue'
import SecondaryButtonVue from '~/components/ui/SecondaryButton.vue'
import AltButtonVue from '~/components/ui/AltButton.vue'

export default {
  inheritAttrs: false,
}
</script>

<script setup lang="ts">
interface Props {
  danger?: boolean
  warn?: boolean
  success?: boolean
  primary?: boolean
  secondary?: boolean
  alt?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  danger: false,
  warn: false,
  success: false,
  primary: false,
  secondary: false,
  alt: false,
})

const buttonType = computed(() => {
  const { danger, warn, success, primary, secondary, alt } = props
  if (danger) // priority line
    return DangerButtonVue
  else if (warn)
    return WarnButtonVue
  else if (success)
    return SuccessButtonVue
  else if (primary)
    return PrimaryButtonVue
  else if (secondary)
    return SecondaryButtonVue
  else if (alt)
    return AltButtonVue
  return PrimaryButtonVue // default
})
</script>

<template>
  <Transition name="component-fade" mode="out-in">
    <component
      :is="buttonType"
      v-bind="$attrs"
    >
      <slot />
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
