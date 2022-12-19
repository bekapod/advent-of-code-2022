#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

#[derive(Debug)]
struct Instruction {
  direction: Direction,
  steps: usize
}

#[derive(Debug, Clone)]
struct Knot {
  position: (i32, i32),
  history: Vec<(i32, i32)>,
  number_of_spots_visited: u32
}

impl Knot {
  fn new() -> Self {
    Knot { position: (0, 0), history: vec![], number_of_spots_visited: 1 }
  }

  fn move_to(&mut self, position: (i32, i32)) {
    self.position = position;

    if !self.history.contains(&position) {
      self.number_of_spots_visited += 1;
    }

    self.history.push(position);
  }
}

fn parse(filename: impl AsRef<Path>) -> Vec<Instruction> {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut instructions = vec![];
  for line in reader.lines() {
    let line = line.expect("could not parse line");

    let direction = match &line[0..1] {
      "U" => Direction::Up,
      "D" => Direction::Down,
      "L" => Direction::Left,
      "R" => Direction::Right,
      _ => panic!("invalid direction")
    };

    let steps = line[2..].parse::<usize>().expect("invalid steps");

    instructions.push(Instruction { direction, steps });
  }

  instructions
}

fn simulate(instructions: &[Instruction], rope_length: usize) -> u32 {
  let mut knots = vec![Knot::new(); rope_length];
  let mut head_position = (0, 0);

  let get_delta = |a: (i32, i32), b: (i32, i32)| ((a.0 - b.0).abs(), (a.1 - b.1).abs());

  for instruction in instructions {
    for _ in 0..instruction.steps {
      let (x, y) = match instruction.direction {
        Direction::Up => (head_position.0, head_position.1 + 1),
        Direction::Down => (head_position.0, head_position.1 - 1),
        Direction::Left => (head_position.0 - 1, head_position.1),
        Direction::Right => (head_position.0 + 1, head_position.1)
      };
      head_position = (x, y);

      let mut knot_before_position = head_position;
      for knot in knots.iter_mut() {
        let mut new_tail_position = (knot.position.0, knot.position.1);
        let (delta_x, delta_y) = get_delta(knot_before_position, knot.position);

        if delta_x > 1 || delta_y > 1 {
          if delta_x >= 1 {
            new_tail_position.0 += if knot_before_position.0 > knot.position.0 { 1 } else { -1 };
          }

          if delta_y >= 1 {
            new_tail_position.1 += if knot_before_position.1 > knot.position.1 { 1 } else { -1 };
          }

          knot.move_to(new_tail_position);
        }

        knot_before_position = knot.position;
      }
    }
  }

  knots.last().expect("no tail").number_of_spots_visited
}

pub fn solve(filename: impl AsRef<Path>) -> (u32, u32) {
  let instructions = parse(filename);
  let first = simulate(&instructions, 1);
  let second = simulate(&instructions, 9);

  (first, second)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_example_1() {
    assert_eq!(solve("example_1.txt"), (13, 1));
  }

  #[test]
  fn solve_example_2() {
    assert_eq!(solve("example_2.txt"), (88, 36));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (6236, 2449));
  }
}