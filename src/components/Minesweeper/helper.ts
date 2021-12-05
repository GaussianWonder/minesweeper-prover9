export interface MinesweeperCell {
  isBomb: boolean
  isRevealed: boolean // isRevealed, isFlagged and isUnknown cannot be true at the same time
  isFlagged: boolean
  isUnknown: boolean
  adjacentBombs: number // shown when isRevealed and not a bomb
  isHint: boolean // no purpose yet, should be active when it is a hint and not revealed
}

export interface MinesweeperBoard {
  rows: number
  cols: number
  cells: MinesweeperCell[][]
}

export const initMinesweeperCell = ({ // used to fill the contents of the cells array
  isBomb: false,
  isRevealed: false,
  isFlagged: false,
  isUnknown: false,
  adjacentBombs: 0,
  isHint: false,
}) as MinesweeperCell

const cartesianProduct = (...a: any[]) => a.reduce((a, b) => a.flatMap((d: any) => b.map((e: any) => [d, e].flat())))

const matrixIndexPairs = ({ rows, cols }: MinesweeperBoard): [number, number][] => {
  return cartesianProduct([...new Array(rows).keys()], [...new Array(cols).keys()])
}

export const neighbordsOf = (i: number, j: number): [number, number][] => {
  return cartesianProduct([i + 1, i - 1, i], [j + 1, j - 1, j])
    .splice(0, 8) // remove pair [i, j] which is not a neighbor of [i, j]
}

export type CellWithAdjacentBombs = ([dx, dy]: [number, number]) => boolean
export const cellWithAdjacentBombs = (cells: MinesweeperCell[][], bombCount = 0): CellWithAdjacentBombs => {
  return ([dx, dy]) => cells[dx][dy].adjacentBombs === bombCount
}

export type OutOfBoundsFilter = ([dx, dy]: [number, number]) => boolean
export const outOfBoundsFilter = (rows: number, cols: number): OutOfBoundsFilter => {
  return ([dx, dy]) => dx >= 0 && dx < rows && dy >= 0 && dy < cols
}

const bombsNear = ({ rows, cols, cells }: MinesweeperBoard, i: number, j: number): number => {
  const outOfBounds = outOfBoundsFilter(rows, cols)
  if (outOfBounds([i, j]) === false) return 0

  const isBomb = ([x, y]: [number, number]): boolean => {
    return cells[x][y].isBomb
  }
  return neighbordsOf(i, j)
    .filter(outOfBounds)
    .filter(isBomb)
    .length
}

export const fillWithBombs = (board: MinesweeperBoard, count: number): MinesweeperBoard => {
  const { cells } = board
  matrixIndexPairs(board)
    .sort(() => (Math.random() > 0.5) ? 1 : -1)
    .slice(0, count)
    .forEach((pair) => {
      const [i, j] = pair
      cells[i][j].isBomb = true
    })

  cells.forEach((row, i) => {
    row.forEach((cell, j) => {
      cells[i][j].adjacentBombs = bombsNear(board, i, j)
    })
  })

  return {
    ...board, // same board
    cells, // new cells
  }
}

export const initCells = (rows: number, cols: number): MinesweeperCell[][] => {
  const cells: MinesweeperCell[][] = []
  for (let i = 0; i < rows; ++i) {
    cells.push(
      new Array<MinesweeperCell>(cols)
        .fill(initMinesweeperCell)
        .map(cell => ({ ...cell })),
    )
  }
  return cells
}

export const initBoard = (rows: number, cols: number, bombs: number): MinesweeperBoard => {
  return fillWithBombs(
    {
      rows,
      cols,
      cells: initCells(rows, cols),
    } as MinesweeperBoard,
    bombs,
  )
}

// this is deprecated and no longer neede, but i'll keep it just in case a fixed array-size is needed in the future
export const padBoard = (board: MinesweeperBoard): MinesweeperBoard => {
  const { rows, cols, cells } = board
  const newCells: MinesweeperCell[][] = []
  const colFiller = new Array(20 - cols).fill({ ...initMinesweeperCell }) as MinesweeperCell[]
  for (let i = 0; i < rows; ++i) {
    for (let j = 0; j < cols; ++j)
      newCells[i] = [...cells[i]].concat(...colFiller)
  }
  for (let i = 1; i <= 12 - rows; ++i)
    newCells.push(new Array(20).fill({ ...initMinesweeperCell }) as MinesweeperCell[])

  return {
    rows,
    cols,
    cells: newCells,
  }
}
