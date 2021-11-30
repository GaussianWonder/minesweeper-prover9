<script setup lang="ts">
import { MinesweeperCell } from './helper'

enum CellStyle {
  Danger = 0,
  Warn,
  Success,
  Primary,
  Secondary,
  Alt,
}

const props = defineProps<{state: MinesweeperCell}>()

const cellStyle = computed(() => {
  const { isRevealed, isBomb, isFlagged, isUnknown, isHint } = props.state
  if (!isRevealed) {
    if (isHint)
      return CellStyle.Success
    if (isFlagged)
      return CellStyle.Alt
    if (isUnknown)
      return CellStyle.Warn
    return CellStyle.Primary
  }
  if (isRevealed) {
    if (isBomb)
      return CellStyle.Danger
    else
      return CellStyle.Secondary
  }
})
</script>

<template>
  <GuardedButton
    class="w-6 h-6 m-1 !p-0"
    :is-tiny="true"
    :danger="cellStyle === CellStyle.Danger"
    :warn="cellStyle === CellStyle.Warn"
    :success="cellStyle === CellStyle.Success"
    :primary="cellStyle === CellStyle.Primary"
    :secondary="cellStyle === CellStyle.Secondary"
    :alt="cellStyle === CellStyle.Alt"
  >
    <!-- isHint -->
    <carbon-ai-results v-if="cellStyle === CellStyle.Success" class="w-full h-full" />
    <!-- isRevealedBomb -->
    <carbon-error-outline v-if="cellStyle === CellStyle.Danger" class="w-full h-full" />
    <!-- isFlagged -->
    <carbon-flag v-if="cellStyle === CellStyle.Alt" class="w-full h-full" />
    <!-- isUnknown -->
    <carbon-unknown v-if="cellStyle === CellStyle.Warn" class="w-full h-full" />
    <!-- isRevealedNumber -->
    <DigitIcon
      v-if="cellStyle === CellStyle.Secondary && state.adjacentBombs > 0"
      class="w-full h-full"
      :digit="state.adjacentBombs"
    />
  </GuardedButton>
</template>
