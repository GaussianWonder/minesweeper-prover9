use std::fmt::Write;
use serde::{Serialize, Deserialize};
use combinations::Combinations;

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

type IndexPair = (i32, i32);
type IndexVector = Vec<IndexPair>;
type Matrix<T> = Vec<Vec<T>>;
type StatementSource = Vec<Vec<IndexPair>>;

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

pub fn get_statements(board: &Board) -> Vec<String> {
  let mut statements: Vec<String> = get_statement_source(board)
    .into_iter()
    .map(|statement| statement_source_to_string(statement))
    .collect();

  statements.dedup(); // the commands should be sorted

  let mut implications: Vec<String> = vec![];

  for statement in &statements {
    let s_imp: Vec<&str> = statement.split("|").collect();
    let imp_cnt = s_imp.len();
    if imp_cnt > 1 {
      for i in 0..imp_cnt {
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