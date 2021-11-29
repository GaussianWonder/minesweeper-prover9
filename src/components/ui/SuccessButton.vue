<template>
  <BaseButton
    class="
      focus:(outline-none ring ring-green-300) text-green-600 bg-green-50 border-green-200 hover:bg-green-100 active:bg-green-200
      dark:(focus:ring-green-700 text-green-200 bg-green-600 border-green-800 hover:bg-green-700 active:bg-green-800)
    "
  >
    <slot />
  </BaseButton>
</template>
