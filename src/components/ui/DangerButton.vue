<template>
  <BaseButton
    class="
      focus:(outline-none ring ring-red-300) text-red-600 bg-red-50 border-red-200 hover:bg-red-100 active:bg-red-200
      dark:(focus:ring-red-700 text-red-200 bg-red-600 border-red-800 hover:bg-red-700 active:bg-red-800)
    "
  >
    <slot />
  </BaseButton>
</template>
