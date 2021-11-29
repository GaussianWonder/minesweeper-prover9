<template>
  <BaseButton
    class="
      focus:(outline-none ring ring-yellow-300) text-yellow-600 bg-yellow-50 border-yellow-200 hover:bg-yellow-100 active:bg-yellow-200
      dark:(focus:ring-yellow-700 text-yellow-200 bg-yellow-600 border-yellow-800 hover:bg-yellow-700 active:bg-yellow-800)
    "
  >
    <slot />
  </BaseButton>
</template>
