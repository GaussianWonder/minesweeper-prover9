<template>
  <BaseButton
    class="
      focus:(outline-none ring ring-gray-300) text-gray-600 bg-gray-50 border-gray-200 hover:bg-gray-100 active:bg-gray-200
      dark:(focus:ring-gray-700 text-gray-200 bg-transparent border-gray-800 hover:bg-gray-700 active:bg-gray-800)
    "
  >
    <slot />
  </BaseButton>
</template>
