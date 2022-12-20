#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug)]
enum Command {
  Noop,
  Add(i32),
}

#[derive(Debug)]
struct Program {
  commands: Vec<Command>,
  output: Vec<i32>,
  screen: Vec<Vec<char>>,
}

impl Program {
  fn new() -> Self {
    Program { commands: vec![], output: vec![], screen: vec![] }
  }

  pub fn add_command(&mut self, command: Command) {
    self.commands.push(command);
  }

  pub fn run(&mut self) {
    let mut screen = vec![vec![' '; 40]; 6];
    let mut output = vec![1];
    let mut cycle = 1;
    let mut current_value = 1;
    for command in &self.commands {
      output.resize(cycle + 1, 0);
      output[cycle] = current_value * cycle as i32;

      let row = ((cycle - 1) as f64 / 40.0).floor() as usize;
      let col = (cycle - 1 as usize) % 40;
      if current_value > 0 {
        if col >= (current_value - 1) as usize && col <= (current_value + 1) as usize {
          screen[row][col] = '█';
        }
      } else {
        if col <= (current_value + 1) as usize {
          screen[row][col] = '█';
        }
      }

      match command {
        Command::Add(value) => {
          current_value = current_value + value;
        },
        Command::Noop => {},
      }
      cycle += 1;
    }

    self.output = output;
    self.screen = screen;
  }

  pub fn draw(&self) {
    for row in &self.screen {
      for col in row {
        print!("{}", col);
      }
      println!();
    }
  }
}

fn parse(filename: impl AsRef<Path>) -> Program {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut program = Program::new();
  for line in reader.lines() {
    let line = line.expect("could not parse line");

    match &line[0..1] {
      "n" => {
        program.add_command(Command::Noop);
      },
      "a" => {
        program.add_command(Command::Noop);
        program.add_command(Command::Add(line[5..].parse::<i32>().expect("invalid number")));
      },
      _ => panic!("invalid command"),
    };
  }

  program
}

pub fn solve(filename: impl AsRef<Path>) -> i32 {
  let mut program = parse(filename);
  program.run();
  let interesting_signal_strengths = [20, 60, 100, 140, 180, 220];
  let mut first = 0;
  for pos in interesting_signal_strengths {
    let signal_strength = program.output[pos as usize];
    first += signal_strength;
  }

  program.draw();

  first
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn run_small_program() {
    let mut program = Program {
      commands: vec![Command::Noop, Command::Noop, Command::Add(3), Command::Noop, Command::Add(-5)],
      output: vec![],
      screen: vec![],
    };
    program.run();
    assert_eq!(
      program.output,
      vec![1, 1, 2, 3, 16, 20]
    );
  }

  #[test]
  fn solve_example() {
    assert_eq!(solve("example.txt"), 13140);
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), 10760);
  }
}