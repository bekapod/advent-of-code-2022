#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path, collections::HashMap,
};

#[derive(Debug)]
struct Entry {
  size: u32,
  path: String
}

fn parse(filename: impl AsRef<Path>) -> Vec<Entry> {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut entries: Vec<Entry> = vec![];
  let mut current_path: Vec<String> = vec![];
  for line in reader.lines() {
    let line = line.expect("could not parse line");

    match line.chars().next().expect("empty line") {
      // command
      '$' => {
        let parts = line.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
        if parts[1] == "ls" {
          // noop
        } else if parts[1] == "cd" {
          let path = &parts[2];

          if path == ".." {
            current_path.pop();
          } else if path == "/" {
            current_path.push(String::from("/root"));
          } else {
            current_path.push(path.to_string());
          }
        }
      }
      // file
      '0'..='9' => {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let entry = Entry {
          size: parts[0].parse().expect("not a number"),
          path: current_path.join("/")
        };
        entries.push(entry)
      }
      // nothing
      _ => {}
    }
  }

  entries
}

pub fn solve(filename: impl AsRef<Path>) -> (u32, u32) {
  let entries = parse(filename);
  let mut directories: HashMap<String, u32> = HashMap::new();

  for entry in entries {
    let dirs = entry.path.split('/').collect::<Vec<&str>>();
    let mut current_path: Vec<&str> = vec![];
    for dir in dirs {
      current_path.push(dir);
      directories.entry(current_path.join("/")).and_modify(|e| *e += entry.size).or_insert(entry.size);
    }
  }

  let all_small_directories = directories.clone().into_iter().filter(|(_, size)| *size <= 100000).map(|(_, size)| size).sum::<u32>();

  let unused_space = 70000000 - directories.clone().get_key_value("/root").unwrap().1;
  let space_needed = 30000000 - unused_space;
  let smallest_directory_to_delete_size = directories.clone().into_iter().filter(|(_, size)| *size >= space_needed).map(|(_, size)| size).min().expect("no directory small enough");

  (all_small_directories, smallest_directory_to_delete_size)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_example() {
    assert_eq!(solve("example.txt"), (95437, 24933642));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (1428881, 10475598));
  }
}