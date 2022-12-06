#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};


fn parse(filename: impl AsRef<Path>) -> Vec<char> {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  reader.lines().next().expect("no lines").expect("could not read line").chars().collect()
}

pub fn solve(filename: impl AsRef<Path>) -> (u32, u32) {
  let message = parse(filename);

  let get_position = |packet_size: u8| {
    let mut position = packet_size as u32;
    for window in message.windows(packet_size as usize) {
      let mut uniques: Vec<&char> = Vec::new();
      for c in window {
        if !uniques.contains(&c) {
          uniques.push(c);
        }
      }

      if uniques.len() == window.len() {
        break
      }

      position += 1;
    }

    position
  };

  (get_position(4), get_position(14))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_example_1() {
    assert_eq!(solve("example_1.txt"), (7, 19));
  }

  #[test]
  fn solve_example_2() {
    assert_eq!(solve("example_2.txt"), (5, 23));
  }

  #[test]
  fn solve_example_3() {
    assert_eq!(solve("example_3.txt"), (6, 23));
  }

  #[test]
  fn solve_example_4() {
    assert_eq!(solve("example_4.txt"), (10, 29));
  }

  #[test]
  fn solve_example_5() {
    assert_eq!(solve("example_5.txt"), (11, 26));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (1647, 2447));
  }
}