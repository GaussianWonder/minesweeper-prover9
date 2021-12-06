#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod Minesweeper;
use combinations::Combinations;

#[tauri::command]
fn prover9_request(board: Minesweeper::Board, message: String) {
  let statements = Minesweeper::get_statements(&board);
  println!("{:#?}", statements);
  Minesweeper::make_input_file(statements);
  let output: String = Minesweeper::execute_input_file();
  let stateful_cells: Vec<Minesweeper::Mace4Model> = Minesweeper::parse_mace4_output(output, board);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![prover9_request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
