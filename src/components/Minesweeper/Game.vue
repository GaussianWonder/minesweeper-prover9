<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { padBoard } from './helper'
import { useGameStore } from '~/stores/game'
const gameState = useGameStore()

type RequestMessage = 'prover9' | 'mace4'

const canAskProver9 = computed(() => {
  const { isFresh, isGameOver } = gameState
  return !isFresh && !isGameOver
})

const launchRequest = (message: RequestMessage) => {
  const { board } = gameState
  if (board) {
    invoke('prover9_request', {
      board: padBoard(board),
      message,
    })
  }
}

</script>

<template>
  <div class="flex flex-col items-center gap-1">
    <Config />
    <Board class="mt-5" />
    <div
      v-if="gameState.isGameOver"
      class="select-none"
    >
      <p class="text-gray-800 dark:text-gray-200">
        The Game is Over
      </p>
      <p class="text-red-800 dark:text-red-200">
        Reset the board to continue
      </p>
    </div>
    <div v-if="canAskProver9">
      <SuccessButton
        class="mx-1"
        @click.prevent="launchRequest('prover9')"
      >
        Mace4
      </SuccessButton>
      <WarnButton
        class="mx-1"
        @click.prevent="launchRequest('mace4')"
      >
        Prover9
      </WarnButton>
    </div>
  </div>
</template>
