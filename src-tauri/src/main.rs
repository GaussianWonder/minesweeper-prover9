#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod Minesweeper;

#[tauri::command]
fn prover9_request(board: Minesweeper::Board, message: String) -> Vec<Minesweeper::IndexedCell> {
  if Minesweeper::is_finished(&board) {
    vec![]
  }
  else {
    let statements = Minesweeper::get_statements(&board);
    if statements.len() == 0 {
      vec![]
    }
    else {
      Minesweeper::make_input_file(statements);
      Minesweeper::parse_mace4_output(Minesweeper::execute_input_file(), board)
    }
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![prover9_request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
