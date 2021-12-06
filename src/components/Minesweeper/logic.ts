import BoardVue from './Board.vue'
import { MinesweeperBoard, MinesweeperCell, neighbordsOf, outOfBoundsFilter, cellWithAdjacentBombs } from './helper'

export const forceReveal = (cell: MinesweeperCell): MinesweeperCell => {
  return {
    ...cell,
    isRevealed: true,
    isHint: false,
    isFlagged: false,
    isUnknown: false,
  } as MinesweeperCell
}

export const reveal = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isFlagged || cell.isUnknown)
    return cell

  return {
    ...cell,
    isRevealed: true,
    isHint: false,
  } as MinesweeperCell
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const forceBoardReveal = (board: MinesweeperBoard, i: number, j: number) => {
  for (let i = 0; i < board.rows; ++i) {
    for (let j = 0; j < board.cols; ++j)
      board.cells[i][j] = forceReveal(board.cells[i][j])
  }
}

export const recursiveReveal = (board: MinesweeperBoard, i: number, j: number): void => {
  const boundCheck = outOfBoundsFilter(board.rows, board.cols)
  const revealCheck = ([dx, dy]: [number, number]) => board.cells[dx][dy].isRevealed === false
  const noBombCheck = cellWithAdjacentBombs(board.cells, 0)
  const neighborQueue = [[i, j]]
  while (neighborQueue.length > 0) {
    const pair = neighborQueue.pop()
    if (!pair) return
    const [x, y] = pair

    const neighbors = neighbordsOf(x, y)
      .filter(boundCheck)
      .filter(revealCheck)

    neighbors.forEach(([dx, dy]) => {
      board.cells[dx][dy] = reveal(board.cells[dx][dy])
    })

    neighbors
      .filter(noBombCheck)
      .forEach(pair => neighborQueue.push(pair))
  }
}

export const recursiveRevealEmpty = (board: MinesweeperBoard): void => {
  const boundCheck = outOfBoundsFilter(board.rows, board.cols)
  const revealCheck = ([dx, dy]: [number, number]) => board.cells[dx][dy].isRevealed === false

  board.cells.forEach((row, ri) => {
    row.forEach((cell, ci) => {
      const { isRevealed, adjacentBombs, isBomb } = cell
      if (isRevealed && adjacentBombs === 0 && !isBomb) {
        neighbordsOf(ri, ci)
          .filter(boundCheck)
          .filter(revealCheck)
          .forEach(([x, y]) => {
            recursiveReveal(board, x, y)
          })
      }
    })
  })
}

export const unreveal = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isFlagged || cell.isUnknown)
    return cell

  return {
    ...cell,
    isRevealed: false,
    isHint: false,
  } as MinesweeperCell
}

export const revealToggle = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isFlagged || cell.isUnknown)
    return cell

  return {
    ...cell,
    isRevealed: !cell.isRevealed,
    isHint: false,
  }
}

export const flag = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isRevealed)
    return cell

  return {
    ...cell,
    isFlagged: true,
    isUnknown: false,
    isHint: false,
  } as MinesweeperCell
}

export const unflag = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isRevealed)
    return cell

  return {
    ...cell,
    isFlagged: false,
    isUnknown: false,
    isHint: false,
  } as MinesweeperCell
}

export const flagToggle = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isRevealed)
    return cell

  return {
    ...cell,
    isFlagged: !cell.isFlagged,
    isUnknown: false,
    isHint: false,
  }
}

export const markUnknown = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isRevealed)
    return cell

  return {
    ...cell,
    isFlagged: false,
    isUnknown: true,
    isHint: false,
  } as MinesweeperCell
}

export const unmarkUnknown = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isRevealed)
    return cell

  return {
    ...cell,
    isFlagged: false,
    isUnknown: false,
    isHint: false,
  } as MinesweeperCell
}

export const markUnknownToggle = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isRevealed)
    return cell

  return {
    ...cell,
    isFlagged: false,
    isUnknown: !cell.isUnknown,
    isHint: false,
  }
}
