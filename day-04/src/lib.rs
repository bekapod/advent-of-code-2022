#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
  str::FromStr,
  string::ParseError,
  collections::HashSet
};


#[derive(Debug, PartialEq, Clone)]
pub struct ElfPair(HashSet<u32>, HashSet<u32>);
impl ElfPair {
  pub fn has_full_overlap(&self) -> bool {
    if self.0.is_subset(&self.1) || self.1.is_subset(&self.0) {
      return true;
    }

    false
  }

  pub fn has_partial_overlap(&self) -> bool {
    if self.0.intersection(&self.1).count() > 0 {
      return true;
    }

    false
  }
}
impl FromStr for ElfPair {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let str_to_room_range = |s: &str| -> HashSet<u32> {
      let mut rooms = s.split("-")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().expect("Failed to parse room range"));
      let start = rooms.next().expect("No start room");
      let end = rooms.next().expect("No end room");
      let mut rooms = HashSet::new();

      for room in start..=end {
        rooms.insert(room);
      }

      rooms
    };

    let mut split_into_pairs = s.split(",");
    Ok(ElfPair(str_to_room_range(split_into_pairs.next().expect("no room range for first elf")), str_to_room_range(split_into_pairs.next().expect("no room range for second elf"))))
  }
}


fn parse(filename: impl AsRef<Path>) -> Vec<ElfPair> {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut pairs = Vec::new();
  for line in reader.lines() {
    let line = line.expect("could not parse line");
    let pair = ElfPair::from_str(&line).expect("could not parse rucksack");
    pairs.push(pair);
  }

  pairs
}

pub fn solve(filename: impl AsRef<Path>) -> (usize, usize) {
  let pairs = parse(filename);
  let number_of_fully_overlapping_pairs = pairs.iter().filter(|pair| pair.has_full_overlap()).count();
  let number_of_partially_overlapping_pairs = pairs.iter().filter(|pair| pair.has_partial_overlap()).count();

  (number_of_fully_overlapping_pairs, number_of_partially_overlapping_pairs)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_example() {
    assert_eq!(solve("example.txt"), (2, 4));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (550, 931));
  }
}