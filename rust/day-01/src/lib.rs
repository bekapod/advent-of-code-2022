#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ElfFoodBag(Vec<u32>);

impl ElfFoodBag {
  pub fn new() -> ElfFoodBag {
    return ElfFoodBag(Vec::new());
  }

  pub fn insert(&mut self, item: u32) {
    self.0.push(item);
  }

  pub fn get_total_number_of_calories(&self) -> u32 {
    return self.0.iter().sum();
  }
}

impl Ord for ElfFoodBag {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    return self.get_total_number_of_calories().cmp(&other.get_total_number_of_calories());
  }
}

impl PartialOrd for ElfFoodBag {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    return Some(self.cmp(other));
  }
}

pub fn solve(filename: impl AsRef<Path>) -> (u32, u32) {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut bags = Vec::new();
  let mut bag = ElfFoodBag::new();
  for line in reader.lines() {
    let line = line.expect("could not parse line");
    if line.is_empty() {
      bags.push(bag);
      bag = ElfFoodBag::new();
    } else {
      let calories = line.split(" ").last().unwrap().parse::<u32>().unwrap();
      bag.insert(calories);
    }
  }
  // add the last bag
  bags.push(bag);
  bags.sort();

  let max_calories_in_a_bag = bags.iter().max().unwrap().get_total_number_of_calories();
  let top_three_bags_total = bags.iter().rev().take(3).map(|bag| bag.get_total_number_of_calories()).sum::<u32>();

  (max_calories_in_a_bag, top_three_bags_total)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_first_example() {
    assert_eq!(solve("example.txt"), (24000, 45000));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (66186, 196804));
  }
}