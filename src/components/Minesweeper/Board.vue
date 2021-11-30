<script setup lang="ts">
import { MinesweeperBoard, initBoard } from './helper'
import { reveal, flagToggle, markUnknownToggle, recursiveReveal, revealAll } from './logic'
import { useGameStore } from '~/stores/game'

const gameState = useGameStore()

const board = ref<MinesweeperBoard>(initBoard(gameState.config.row, gameState.config.col, gameState.config.bomb))

watchEffect(() => {
  const { row, col, bomb } = gameState.config
  // when this code is executed, this means that the game state inside the store has been changed => reset the game
  board.value = initBoard(row, col, bomb)
  // TODO reset logic
})

const revealCell = (i: number, j: number) => {
  const cell = board.value.cells[i][j]
  const { adjacentBombs, isBomb, isFlagged, isUnknown } = cell

  if (!isBomb) {
    if (adjacentBombs === 0)
      recursiveReveal(board.value, i, j)
    else
      board.value.cells[i][j] = reveal(cell)
  }
  else if (!isFlagged && !isUnknown) {
    board.value = revealAll(board.value)
  }
}

const flagCell = (i: number, j: number) => {
  board.value.cells[i][j] = flagToggle(board.value.cells[i][j])
}

const questionCell = (i: number, j: number) => {
  board.value.cells[i][j] = markUnknownToggle(board.value.cells[i][j])
}

const ignore = () => {}
</script>

<template>
  <div @contextmenu.prevent="ignore">
    <div
      v-for="(row, ri) in board.cells"
      :key="`cell_row_${ri}`"
      class="flex flex-row"
    >
      <Cell
        v-for="(cell, ci) in row"
        :key="`cell_col_${ci}`"
        :state="cell"
        @contextmenu.prevent="flagCell(ri, ci)"
        @click.left.prevent="revealCell(ri, ci)"
        @click.middle.prevent="questionCell(ri, ci)"
      >
      </Cell>
    </div>
  </div>
</template>
