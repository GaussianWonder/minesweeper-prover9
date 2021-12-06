<script setup lang="ts">
import { MinesweeperCell, MinesweeperBoard, initBoard } from './helper'
import { reveal, flagToggle, markUnknownToggle, recursiveReveal, forceBoardReveal, recursiveRevealEmpty } from './logic'
import { useGameStore } from '~/stores/game'

const gameState = useGameStore()

const board = ref<MinesweeperBoard>(initBoard(gameState.config.row, gameState.config.col, gameState.config.bomb))

watchEffect(() => {
  const { row, col, bomb } = gameState.config
  // when this code is executed, this means that the game state inside the store has been changed => reset the game
  board.value = initBoard(row, col, bomb)
  gameState.setIsFreshState(true)
  gameState.setBoard(board.value)
})

watchEffect(() => {
  const { cellsToUpdate, updateCells } = gameState
  if (cellsToUpdate.length) {
    cellsToUpdate.forEach(([[i, j], cell]) => {
      board.value.cells[i][j] = {
        ...cell,
        adjacentBombs: board.value.cells[i][j].adjacentBombs,
      }
    })
    updateCells([]) // so that we 100% get notified the next time we need to update
    recursiveRevealEmpty(board.value)
  }
})

const revealCell = (i: number, j: number) => {
  if (board.value.cells[i][j].isRevealed)
    return

  const cell = board.value.cells[i][j]
  const { adjacentBombs, isBomb, isFlagged, isUnknown } = cell

  if (!isBomb) {
    if (adjacentBombs === 0)
      recursiveReveal(board.value, i, j)
    else
      board.value.cells[i][j] = reveal(cell)
  }
  else if (!isFlagged && !isUnknown) {
    forceBoardReveal(board.value, i, j)
    gameState.setIsGameOver(true)
  }

  gameState.setIsFreshState(false)
  gameState.setBoard(board.value)
}

const flagCell = (i: number, j: number) => {
  if (board.value.cells[i][j].isRevealed)
    return
  board.value.cells[i][j] = flagToggle(board.value.cells[i][j])
  gameState.setBoard(board.value)
}

const questionCell = (i: number, j: number) => {
  if (board.value.cells[i][j].isRevealed)
    return
  board.value.cells[i][j] = markUnknownToggle(board.value.cells[i][j])
  gameState.setBoard(board.value)
}

const breakTheIce = () => {
  const chosen = board.value.cells
    .flatMap((row, ri) => row
      .map((cell, ci) =>
        ({
          bombs: cell.adjacentBombs + (cell.isBomb ? 1 : 0),
          matrixIndex: [ri, ci],
        }),
      )
      .filter(c => c.bombs === 0),
    )
    .sort(() => (Math.random() > 0.5) ? 1 : -1)
    .pop()

  if (!chosen) return
  const { matrixIndex } = chosen
  revealCell(matrixIndex[0], matrixIndex[1])
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
    <div
      v-if="gameState.isFresh"
      class=""
    >
      <SuccessButton @click.prevent="breakTheIce">
        Break the ice
      </SuccessButton>
    </div>
  </div>
</template>
