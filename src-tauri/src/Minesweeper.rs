use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
  isBomb: bool,
  isRevealed: bool,
  isFlagged: bool,
  isUnknown: bool,
  adjacentBombs: i32,
  isHint: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
  rows: i32,
  cols: i32,
  cells: Vec<Vec<Cell>>
}