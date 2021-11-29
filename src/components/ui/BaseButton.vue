<script setup lang="ts">
interface Props {
  href?: string
  isTiny?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  href: undefined,
  isTiny: false,
})

const componentType = computed(() => {
  /**
   ** There is also an <input type="submit" /> which acts like a button
    * However it is shadowed by <button type="submit" /> which does the same thing
    *  and also accepts content
    */
  return props.href ? 'a' : 'button'
})

const padding = computed(() => {
  return props.isTiny ? 'px-1' : 'px-4 py-2'
})
</script>

<template>
  <component
    :is="componentType"
    :href="props.href"
    class="transition select-none rounded-md text-xs font-semibold tracking-widest border shadow"
    :class="`${padding}`"
  >
    <slot />
  </component>
</template>
