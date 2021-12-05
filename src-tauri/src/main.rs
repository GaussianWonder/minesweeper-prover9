#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod Minesweeper;

#[tauri::command]
fn prover9_request(board: Minesweeper::Board, message: String) {
  println!("Requested {} from {:#?}", message, board);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![prover9_request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
