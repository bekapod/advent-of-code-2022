#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
  str::FromStr, string::ParseError,
};
use priority::Priority;
use priority_derive::Priority;

#[derive(Debug, PartialEq, Priority, Clone)]
pub struct Item(String);
impl FromStr for Item {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Item(s.to_string()))
  }
}
impl PartialEq<str> for Item {
  fn eq(&self, other: &str) -> bool {
    self.0 == other
  }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Rucksack(Vec<Item>, Vec<Item>);

impl Rucksack {
  fn get_duplicate_items(&self) -> Vec<&Item> {
    let mut duplicates = Vec::new();
    for item in &self.0 {
      if self.1.contains(item) && !duplicates.contains(&item) {
        duplicates.push(item);
      }
    }
    duplicates
  }

  pub fn get_priority_of_duplicate_items(&self) -> u32 {
    self.get_duplicate_items().into_iter().map(|item| item.get_priority_of_item()).sum()
  }

  pub fn get_all_items(&self) -> Vec<Item> {
    let mut items = self.0.clone();
    items.extend(self.1.clone());
    items
  }
}
impl FromStr for Rucksack {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let str_to_compartment = |s: &str| -> Vec<Item> {
      s.split("")
        .filter(|s| !s.is_empty())
        .map(|s| Item::from_str(s).expect("Could not parse item"))
        .collect()
    };

    let split_in_half = s.split_at(s.len() / 2);
    Ok(Rucksack(str_to_compartment(split_in_half.0), str_to_compartment(split_in_half.1)))
  }
}

#[derive(Debug)]
pub struct ElfGroup(Rucksack, Rucksack, Rucksack);
impl ElfGroup {
  pub fn find_badge(&self) -> Option<Item> {
    // TODO: can rucksack implement iterator?
    for item in &self.0.get_all_items() {
      if self.1.get_all_items().contains(item) && self.2.get_all_items().contains(item) {
        return Some(item.clone());
      }
    }

    None
  }
}


fn parse(filename: impl AsRef<Path>) -> Vec<Rucksack> {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut rucksacks = Vec::new();
  for line in reader.lines() {
    let line = line.expect("could not parse line");
    let rucksack = Rucksack::from_str(&line).expect("could not parse rucksack");
    rucksacks.push(rucksack);
  }

  rucksacks
}

pub fn solve(filename: impl AsRef<Path>) -> (u32, u32) {
  let rucksacks = parse(filename);

  let first = rucksacks.clone().into_iter().map(|r| r.get_priority_of_duplicate_items()).sum();

  let groups = rucksacks.clone().chunks(3).map(|r| ElfGroup(r[0].clone(), r[1].clone(), r[2].clone())).collect::<Vec<_>>();
  let second: u32 = groups.into_iter().map(|g| g.find_badge().expect("could not find badge").get_priority_of_item()).sum();

  (first, second)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_example() {
    assert_eq!(solve("example.txt"), (157, 70));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (7917, 2585));
  }
}