import { acceptHMRUpdate, defineStore } from 'pinia'

export const useGameStore = defineStore('game', () => {
  /**
   * Current config of the game.
   */
  const config = ref({
    row: 9,
    col: 9,
    bomb: 10,
  })

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

  return {
    config,
    setRowCount,
    setColCount,
    setBombCount,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useGameStore, import.meta.hot))
