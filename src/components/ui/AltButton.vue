<template>
  <BaseButton
    class="
      focus:(outline-none ring ring-indigo-300) text-indigo-600 bg-indigo-50 border-indigo-200 hover:bg-indigo-100 active:bg-indigo-200
      dark:(focus:ring-indigo-700 text-indigo-200 bg-indigo-600 border-indigo-800 hover:bg-indigo-700 active:bg-indigo-800)
    "
  >
    <slot />
  </BaseButton>
</template>
