#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug)]
struct Forest {
  width: usize,
  height: usize,
  pub rows: Vec<Vec<u8>>,
}
impl Forest {
  pub fn is_tree_visible(&self, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || y >= self.rows.len() || x >= self.rows[y].len() {
      return true;
    }
  
    let tree = self.rows[y][x];
    let taller_trees_above = &self.rows[0..=y-1].into_iter().map(|row| row[x]).filter(|tree_above| tree_above >= &tree).collect::<Vec<u8>>();
    let taller_trees_below = &self.rows[y+1..].into_iter().map(|row| row[x]).filter(|tree_below| tree_below >= &tree).collect::<Vec<u8>>();
    let taller_trees_left = &self.rows[y][0..=x-1].into_iter().filter(|tree_left| tree_left >= &&tree).collect::<Vec<&u8>>();
    let taller_trees_right = &self.rows[y][x+1..].into_iter().filter(|tree_right| tree_right >= &&tree).collect::<Vec<&u8>>();

    if taller_trees_above.is_empty() || taller_trees_below.is_empty() || taller_trees_left.is_empty() || taller_trees_right.is_empty() {
      return true;
    }

    false
  }

  pub fn calculate_score_for_position(&self, x: usize, y: usize) -> u32 {
    let mut score = 1;
    let tree = self.rows[y][x];

    if y > 0 {
      let mut number_of_trees_above = 0;
      let trees_above = &self.rows[..=y-1].into_iter().map(|row| row[x]).rev().collect::<Vec<u8>>();
      for tree_above in trees_above {
        number_of_trees_above += 1;
        if tree_above >= &tree {
          break;
        }
      }
      score *= number_of_trees_above;
    } else {
      score *= 0;
    }

    if y < self.rows.len() - 1 {
      let mut number_of_trees_below = 0;
      let trees_below = &self.rows[y+1..].into_iter().map(|row| row[x]).collect::<Vec<u8>>();
      for tree_below in trees_below {
        number_of_trees_below += 1;
        if tree_below >= &&tree {
          break;
        }
      }
      score *= number_of_trees_below;
    } else {
      score *= 0;
    }

    if x > 0 {
      let mut number_of_trees_left = 0;
      let trees_left = &self.rows[y][..=x-1].into_iter().rev().collect::<Vec<&u8>>();
      for tree_left in trees_left {
        number_of_trees_left += 1;
        if tree_left >= &&tree {
          break;
        }
      }
      score *= number_of_trees_left;
    } else {
      score *= 0;
    }

    if x > 0 {
      let mut number_of_trees_right = 0;
      let trees_right = &self.rows[y][x+1..];
      for tree_right in trees_right {
        number_of_trees_right += 1;
        if tree_right >= &&tree {
          break;
        }
      }
      score *= number_of_trees_right;
    } else {
      score *= 0;
    }

    score
  }

  pub fn get_best_score(&self) -> u32 {
    self.rows.iter().enumerate().fold(0, |best_score, (y, row)| {
      let mut best_row_score = 1;
      for (x, _tree) in row.iter().enumerate() {
        let score = self.calculate_score_for_position(x, y);
        if score > best_row_score {
          best_row_score = score;
        }
      };

      if best_row_score > best_score {
        best_row_score
      } else {
        best_score
      }
    })
  }
}

fn parse(filename: impl AsRef<Path>) -> Forest {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut forest: Forest = Forest { rows: vec![], width: 0, height: 0 };
  for line in reader.lines() {
    let line = line.expect("could not parse line");

    forest.width = line.len();
    forest.height += 1;
    forest.rows.push(line.chars().map(|c| c.to_string().parse().expect("could not parse tree height")).collect());
  }

  forest
}

pub fn solve(filename: impl AsRef<Path>) -> (u32, u32) {
  let forest = parse(filename);
  let number_of_visible_trees = forest.rows.iter().enumerate().fold(0, |trees_visible, (y, row)| {
    let trees_visible = trees_visible + row.iter().enumerate().fold(0, |trees_visible, (x, _tree)| {
      if forest.is_tree_visible(x, y) {
        trees_visible + 1
      } else {
        trees_visible
      }
    });

    trees_visible
  });

  let best_score = forest.get_best_score();

  (number_of_visible_trees, best_score)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_score_for_position_a() {
    let forest = parse("example.txt");
    assert_eq!(forest.calculate_score_for_position(2, 1), 4);
  }

  #[test]
  fn calculate_score_for_position_b() {
    let forest = parse("example.txt");
    assert_eq!(forest.calculate_score_for_position(2, 3), 8);
  }

  #[test]
  fn calculate_score_for_position_c() {
    let forest = parse("example.txt");
    assert_eq!(forest.calculate_score_for_position(3, 4), 0);
  }

  #[test]
  fn calculate_score_for_position_d() {
    let forest = parse("example.txt");
    assert_eq!(forest.calculate_score_for_position(3, 2), 2);
  }

  #[test]
  fn calculate_score_for_position_e() {
    let forest = parse("input.txt");
    assert_eq!(forest.calculate_score_for_position(20, 7), 6);
  }

  #[test]
  fn calculate_score_for_position_f() {
    let forest = parse("input.txt");
    assert_eq!(forest.calculate_score_for_position(39, 11), 12);
  }

  #[test]
  fn calculate_score_for_position_g() {
    let forest = parse("input.txt");
    assert_eq!(forest.calculate_score_for_position(55, 13), 1);
  }

  #[test]
  fn calculate_score_for_position_h() {
    let forest = parse("input.txt");
    assert_eq!(forest.calculate_score_for_position(97, 98), 0);
  }

  #[test]
  fn solve_example() {
    assert_eq!(solve("example.txt"), (21, 8));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (1679, 536625));
  }
}