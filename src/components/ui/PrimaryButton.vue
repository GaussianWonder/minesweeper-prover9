<template>
  <BaseButton
    class="
      focus:(outline-none ring ring-gray-700) text-gray-50 bg-gray-800 border-gray-900 hover:bg-gray-700 active:bg-gray-600
      dark:(focus:ring-gray-700 text-gray-200 bg-gray-600 border-gray-800 hover:bg-gray-700 active:bg-gray-800)
    "
  >
    <slot />
  </BaseButton>
</template>
