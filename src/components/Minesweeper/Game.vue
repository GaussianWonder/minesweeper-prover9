<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { padBoard } from './helper'
import { useGameStore } from '~/stores/game'
const gameState = useGameStore()

const canAskProver9 = computed(() => {
  const { isFresh } = gameState
  return !isFresh
})

const launchProver9Request = () => {
  const { board } = gameState
  if (board) {
    invoke('prover9_request', {
      board: padBoard(board),
    })
  }
}

</script>

<template>
  <div class="flex flex-col items-center gap-1">
    <Config />
    <Board class="mt-5" />
    <div v-if="canAskProver9">
      <SuccessButton
        @click.prevent="launchProver9Request"
      >
        Prover9
      </SuccessButton>
    </div>
  </div>
</template>
