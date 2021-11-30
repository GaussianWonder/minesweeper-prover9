#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MinesweeperCell {
  isBomb: bool,
  isRevealed: bool,
  isFlagged: bool,
  isUnknown: bool,
  adjacentBombs: i32,
  isHint: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct MinesweeperBoard {
  rows: i32,
  cols: i32,
  cells: [[MinesweeperCell; 20]; 12]
}

#[tauri::command]
fn prover9_request(board: MinesweeperBoard) {
  println!("{:?}", board);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![prover9_request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
