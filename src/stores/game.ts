import { acceptHMRUpdate, defineStore } from 'pinia'
import { MinesweeperBoard, MinesweeperCell } from '~/components/Minesweeper/helper'

export const useGameStore = defineStore('game', () => {
  /**
   * Current config of the game.
   */
  const config = ref({
    row: 9,
    col: 9,
    bomb: 10,
  })
  const isFresh = ref(false)
  const board = ref<MinesweeperBoard | undefined>(undefined)
  const isGameOver = ref(false)

  /**
   * Partial changes to the current game config
   */
  function setRowCount(count: number) {
    config.value = {
      ...config.value,
      row: count,
    }
  }
  function setColCount(count: number) {
    config.value = {
      ...config.value,
      col: count,
    }
  }
  function setBombCount(count: number) {
    config.value = {
      ...config.value,
      bomb: count,
    }
  }

  function setIsFreshState(state: boolean) {
    isFresh.value = state
  }

  function setIsGameOver(state: boolean) {
    isGameOver.value = state
  }

  function setBoard(newBoard: MinesweeperBoard) {
    board.value = newBoard
  }

  const cellsToUpdate = ref<[[number, number], MinesweeperCell][]>([])
  function updateCells(cells: [[number, number], MinesweeperCell][]) {
    cellsToUpdate.value = cells
  }

  return {
    config,
    isFresh,
    board,
    isGameOver,
    setRowCount,
    setColCount,
    setBombCount,
    setIsFreshState,
    setBoard,
    setIsGameOver,
    cellsToUpdate,
    updateCells,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useGameStore, import.meta.hot))
