use std::fmt::Write;
use std::fs;
use std::process::{Command};
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use serde::{Serialize, Deserialize};
use combinations::Combinations;

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct Cell {
  isBomb: bool,
  isRevealed: bool,
  isFlagged: bool,
  isUnknown: bool,
  adjacentBombs: i32,
  isHint: bool
}

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct Board {
  rows: i32,
  cols: i32,
  cells: Vec<Vec<Cell>>
}

pub type IndexPair = (i32, i32);
pub type IndexVector = Vec<IndexPair>;
type Matrix<T> = Vec<Vec<T>>;
type StatementSource = Vec<Vec<IndexPair>>;
type Statement = String;
type Statements = Vec<String>;
pub type IndexedCell = (IndexPair, Cell);

pub fn is_finished(board: &Board) -> bool {
  for row in &board.cells {
    for cell in row {
      if cell.isBomb == true && cell.isFlagged == false {
        return false
      }
    }
  }
  return true
}

// directions for all possible neighbors
const RELATIVE_NEIGHBORS: [IndexPair; 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

// get neighbors of index
pub fn get_neighbors(index: IndexPair, rows: i32, cols: i32) -> IndexVector {
  RELATIVE_NEIGHBORS
    .to_vec()
    .into_iter()
    .map(|(x, y)| (index.0+x, index.1+y)) // index's neighbors
    .filter(|&(x, y)| x >= 0 && x < rows && y >= 0 && y < cols) // inner bounded neighbors
    .collect()
}

pub fn get_frontier(board: &Board) -> IndexVector {
  let cols = board.cols;
  let rows = board.rows;

  let mut stack: IndexVector = vec![];
  let mut visited: Matrix<bool> = vec![vec![false; cols as usize]; rows as usize];
  let mut frontier: IndexVector = vec![];
  let mut in_frontier: Matrix<bool> = vec![vec![false; cols as usize]; rows as usize];

  for i in 0..rows {
    for j in 0..cols {

      if visited[i as usize][j as usize] == false {
        stack.push((i, j));
        while !stack.is_empty() {
          if let Some(current) = stack.pop() {
            visited[current.0 as usize][current.1 as usize] = true;
            let neighbors: Vec<IndexPair> = get_neighbors(current, rows, cols)
              .into_iter()
              .filter(|&(x, y)| board.cells[x as usize][y as usize].isRevealed == false)
              .collect();

            let subjects: Vec<IndexPair> = (&neighbors)
              .to_vec()
              .into_iter()
              .map(|pair: IndexPair| -> Vec<IndexPair> {
                get_neighbors(pair, rows, cols)
                  .into_iter()
                  .filter(|&(x, y)|
                    match board.cells[x as usize][y as usize] {
                      Cell {
                        isRevealed: true,
                        adjacentBombs,
                        isBomb: false,
                        ..
                      } => adjacentBombs > 0,
                      Cell {..} => false
                    }
                  )
                  .collect()
                }
              )
              .flatten()
              .collect();

            for (x, y) in subjects {
              if !in_frontier[x as usize][y as usize] {
                in_frontier[x as usize][y as usize] = true;
                frontier.push((x, y));
              }
            }
      
            for (x, y) in neighbors {
              if !visited[x as usize][y as usize] {
                stack.push((x, y));
              }
            }
          }
        }
      }

    }
  }

  frontier
}

pub fn get_statement_source(board: &Board) -> Vec<StatementSource> {
  let cols = board.cols;
  let rows = board.rows;

  get_frontier(board)
    .into_iter()
    .map(|index|
      (
        index, // the leading hint
        get_neighbors(index, rows, cols) // its unknown neighbors
          .into_iter()
          .filter(|&(x, y)| board.cells[x as usize][y as usize].isRevealed == false)
          .collect()
      )
    )
    .map(|(source, neighbors): (IndexPair, Vec<IndexPair>)| -> StatementSource {
      let variants = board.cells[source.0 as usize][source.1 as usize].adjacentBombs as usize;
      println!("{:?} having {} variants to choose from {:?}", source, variants, neighbors);
      if neighbors.len() <= variants {
        vec![neighbors]
      }
      else {
        Combinations::new(
          neighbors, // candidates for combinations to take from
          board.cells[source.0 as usize][source.1 as usize].adjacentBombs as usize // number of possible bombs at a time
        ).collect()
      }
    })
    .collect()
}

pub fn statement_source_to_string(source: StatementSource) -> String {
  println!("converting {:?}", source);
  let mut or_cluster: String = "".to_string();
  for combination in &source { // or cluster
    // everything here will be 'or'ed with other elements
    let mut and_cluster: String = "".to_string();
    for variant in combination { // and cluster
      // everything here will be 'and'ed with the other elements
      write!(and_cluster, "{}", format!("m{}{}&", variant.0, variant.1)).unwrap();
    }
    and_cluster.pop(); // delete last redundant &
    if combination.len() == 1 {
      write!(or_cluster, "{}|", and_cluster).unwrap();
    }
    else {
      write!(or_cluster, "({})|", and_cluster).unwrap();
    }
  }
  or_cluster.pop(); // delete last redundant |
  or_cluster
}

pub fn get_statements(board: &Board) -> Statements {
  let mut statements: Statements = get_statement_source(board)
    .into_iter()
    .map(|statement| statement_source_to_string(statement))
    .collect();

  statements.dedup(); // the commands should be sorted

  let mut implications: Statements = vec![];

  for statement in &statements {
    let s_imp: Vec<&str> = statement.split("|").collect(); // split all the or statemenets
    let imp_cnt = s_imp.len();
    if imp_cnt > 1 { // if more than one or statement
      for i in 0..imp_cnt { // imply that if one is true, the others are not
        let mut imp: String = "".to_string();

        let subject = s_imp[i];
        let mut all_but_subject = s_imp.to_vec();
        all_but_subject.swap_remove(i as usize);

        let mut abs_condition: String = "".to_string();
        for abs in all_but_subject {
          write!(abs_condition, "{}|", abs).unwrap();
        }
        abs_condition.pop();

        write!(imp, "{} -> -({})", subject, abs_condition);
        implications.push(imp);
      }
    }
  }
 
  statements.append(&mut implications);

  statements
}

pub fn make_input_file(statements: Statements) {
  let prepared_statements: String = statements.join(".\n"); // end statements and new line them, keep in mind the last one does not have a .
  let mut file_contents: String = "".to_string();
  write!(file_contents, "
assign(max_seconds, 30).

formulas(assumptions).
{}.
end_of_list.

formulas(goals).
end_of_list.
    ",
    prepared_statements,
  );
  fs::write("minesweeper.in", &mut file_contents).expect("Unable to write file!");
}

pub fn execute_input_file() -> String {
  // mace4 -c -m -1 -n 2 -f minesweeper.in > minesweeper.out
  let output = Command::new("mace4")
    .arg("-c")
    .arg("-m")
    .arg("-1")
    .arg("-n")
    .arg("2")
    .arg("-f")
    .arg("minesweeper.in")
    .output()
    .expect("mace4 is not accessible");
  let mut stdout = String::from_utf8(output.stdout).unwrap();
  fs::write("minesweeper.out", &mut stdout).expect("Unable to write file!");
  stdout
}

pub fn reduce_cell_state_variation(states: Vec<Cell>) -> Cell {
  let mut is_bomb = true;
  let mut is_not_bomb = true;
  for state in states {
    if state.isBomb == false {
      is_bomb = false; // all states should be bombs for this to be 100% a bomb
    }
    if state.isBomb == true { // all states should be safe for this to be 100% safe
      is_not_bomb = false;
    }
  }
  let is_unknown = !(is_bomb ^ is_not_bomb);

  Cell {
    isBomb: is_bomb,
    isRevealed: is_not_bomb,
    isFlagged: is_bomb,
    isUnknown: is_unknown,
    adjacentBombs: 0,
    isHint: false,
  }
}

// Regex to find model bodies 
// interpretation\( \d+, \[number=(\d+), seconds=\d+], \[((?:\s+relation\(m\d+, \[ \d+ \]\),?)+)\s+\]\)\.
// group1 -> index
// group2 -> relation array

// Regex to identify relation
// ((?:relation\(m(\d+), \[ (\d+) \]\),?)+)
// group1 -> matrix index
// group2 -> isBomb value
pub fn parse_mace4_output(output: String, board: Board) -> Vec<IndexedCell> {
  let mut imap = HashMap::<IndexPair, Vec<Cell>>::new();

  let i_re = regex::Regex::new(r"interpretation\( \d+, \[number=(\d+), seconds=\d+], \[((?:\s+relation\(m\d+, \[ \d+ \]\),?)+)\s+\]\)\.").unwrap();
  for interpretation in i_re.find_iter(&output) {
    // println!("{:?}: {}", interpretation, interpretation.as_str());
    let r_re = regex::Regex::new(r"((?:relation\(m(\d+), \[ (\d+) \]\),?)+)").unwrap();
    for relation in r_re.find_iter(interpretation.as_str()) {
      let caps = r_re.captures(relation.as_str()).unwrap();
      let encoded_index = caps.get(2).map_or("", |m| m.as_str());
      let encoded_bool = caps.get(3).map_or("", |m| m.as_str());

      let is_bomb: bool = match encoded_bool {
        "1" => true,
        _ => false,
      };

      let index_i: i32 = encoded_index.chars().nth(0).unwrap().to_digit(10).unwrap() as i32;
      let index_j: i32 = encoded_index.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
      let pair: IndexPair = (index_i, index_j);
      let cell = Cell {
        isBomb: is_bomb,
        isRevealed: false,
        isFlagged: false,
        isUnknown: false,
        adjacentBombs: board.cells[index_i as usize][index_j as usize].adjacentBombs,
        isHint: false,
      };

      if let Some(states) = imap.get_mut(&pair) {
        states.push(cell);
      }
      else {
        imap.insert(pair, vec![cell]);
      }
    }
  }

  let mut solved_cells: Vec<IndexedCell> = vec![];
  for (key, value) in imap {
    solved_cells.push((key, reduce_cell_state_variation(value)));
  }
  solved_cells
}

