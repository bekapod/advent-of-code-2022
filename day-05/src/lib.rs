#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug, PartialEq, Clone)]
pub struct CrateStack(Vec<String>);
impl CrateStack {
  pub fn new() -> Self {
    Self(vec![])
  }
  pub fn add_crate_to_bottom(&mut self, crate_name: String) {
    self.0.push(crate_name);
  }

  pub fn add_crate_to_top(&mut self, crate_name: String) {
    self.0.insert(0, crate_name);
  }

  pub fn add_crates_to_top(&mut self, crate_names: Vec<String>) {
    for crate_name in crate_names {
      self.add_crate_to_top(crate_name);
    }
  }

  pub fn shift_crate(&mut self) -> Option<String> {
    if self.0.len() >= 1 {
      return Some(self.0.remove(0));
    } else {
      None
    }
  }

  pub fn shift_crates(&mut self, n: usize) -> Vec<String> {
    let mut crates = vec![];
    if self.0.len() >= n {
     
      for _ in 0..n {
        if let Some(crate_name) = self.shift_crate() {
          crates.push(crate_name);
        }
      }
    }

    crates.reverse();

    crates
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
  number_of_crates_to_move: usize,
  starting_stack: usize,
  ending_stack: usize,
}


#[derive(Debug, PartialEq, Clone)]
pub struct CratePlan(Vec<CrateStack>, Vec<Instruction>);
impl CratePlan {
  pub fn run_9000(&mut self) -> Self {
    for instruction in &self.1 {
      for _ in 0..instruction.number_of_crates_to_move {
        let crate_name = self.0[instruction.starting_stack].shift_crate();
        if let Some(crate_name) = crate_name {
          self.0[instruction.ending_stack].add_crate_to_top(crate_name.to_string());
        }
        
      }
    }

    self.clone()
  }

  pub fn run_9001(&mut self) -> Self {
    for instruction in &self.1 {
      let crate_names = self.0[instruction.starting_stack].shift_crates(instruction.number_of_crates_to_move);
      self.0[instruction.ending_stack].add_crates_to_top(crate_names);
    }

    self.clone()
  }

  pub fn get_top_crates(&self) -> String {
    self.0.iter().map(|stack| {
      let top_crate = stack.0.first();
      if let Some(top_crate) = top_crate {
        top_crate.to_string()
      } else {
        "".to_string()
      }
    }).fold(String::new(), |acc, c| {
      return acc + &c;
    })
  }
}


fn parse(filename: impl AsRef<Path>) -> CratePlan {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut have_all_stacks = false;
  let mut stacks: Vec<CrateStack> = Vec::new();
  let mut instructions = Vec::new();
  for line in reader.lines() {
    let line = line.expect("could not parse line");

    if line.is_empty() {
      have_all_stacks = true;
      continue;
    }

    if !have_all_stacks {
      let l = line;
      let exploded = l.split("");
      let non_empty_crates = exploded.filter(|l| !l.is_empty()).collect::<Vec<&str>>();
      let rows = non_empty_crates.chunks(4);

      // for each crate in chunk, add crate to stack matching index
      for (i, row) in rows.enumerate() {
        if stacks.get(i).is_none() {
          stacks.insert(i, CrateStack::new());
        }

        match row[1].chars().next().expect("could not get first char") {
          'A'..='Z' => stacks[i].add_crate_to_bottom(row[1].to_string()),
          _ => {}
        }
      }
    } else {
      let l = line;
      let exploded = l.split(" ");
      let ns = exploded.fold(vec![], |acc, l| {
        if let Ok(n) = l.parse::<usize>() {
          return [acc, vec![n]].concat();
        }

        acc
      });

      instructions.push(Instruction {
        number_of_crates_to_move: ns[0],
        starting_stack: ns[1] - 1,
        ending_stack: ns[2] - 1,
      });
    }
  }

  CratePlan(stacks, instructions)
}

pub fn solve(filename: impl AsRef<Path>) -> (String, String) {
  let plan = parse(filename);
  let old = plan.clone().run_9000();
  let new = plan.clone().run_9001();
  
  (old.get_top_crates(), new.get_top_crates())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_example() {
    assert_eq!(solve("example.txt"), (String::from("CMZ"), String::from("MCD")));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (String::from("SVFDLGLWV"), String::from("DCVTCVPCL")));
  }
}