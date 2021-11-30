import { MinesweeperBoard, MinesweeperCell, neighbordsOf, outOfBoundsFilter, cellWithAdjacentBombs } from './helper'

export const reveal = (cell: MinesweeperCell): MinesweeperCell => {
  if (cell.isFlagged || cell.isUnknown)
    return cell

  return {
    ...cell,
    isRevealed: true,
    isHint: false,
  } as MinesweeperCell
}

export const revealAll = (board: MinesweeperBoard): MinesweeperBoard => {
  const { cells } = board
  cells.forEach((row, ri) => {
    row.forEach((cell, ci) => {
      cells[ri][ci] = reveal(cell)
    })
  })
  return {
    ...board,
    cells,
  } as MinesweeperBoard
}

export const recursiveReveal = (board: MinesweeperBoard, i: number, j: number): void => {
  const allNeighbors = neighbordsOf(i, j)
    .filter(outOfBoundsFilter(board.rows, board.cols))
    .filter(([dx, dy]) => board.cells[dx][dy].isRevealed === false)

  allNeighbors.forEach(([dx, dy]) => {
    board.cells[dx][dy] = reveal(board.cells[dx][dy])
  })

  allNeighbors.filter(cellWithAdjacentBombs(board.cells)).forEach(([dx, dy]) => {
    setTimeout(() => {
      recursiveReveal(board, dx, dy)
    }, 25)
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
