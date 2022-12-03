#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug, PartialEq)]
enum Choice {
  Rock,
  Paper,
  Scissors
}

#[derive(Debug, PartialEq)]
enum Strategy {
  Win,
  Lose,
  Draw
}

#[derive(Debug, PartialEq)]
pub struct Game((Choice, Choice));

impl Game {
  pub fn get_score(&self) -> u32 {
    let mut score: u32;
    let is_draw = self.0.0 == self.0.1;
    let is_loss = match self.0 {
      (Choice::Rock, Choice::Scissors) => true,
      (Choice::Paper, Choice::Rock) => true,
      (Choice::Scissors, Choice::Paper) => true,
      _ => false,
    };

    if is_draw {
      score = 3;
    } else if is_loss {
      score = 0;
    } else {
      score = 6;
    }

    match self.0.1 {
      Choice::Rock => score = score + 1,
      Choice::Paper => score = score + 2,
      Choice::Scissors => score = score + 3,
    }

    score
  }
}

fn parse(filename: impl AsRef<Path>) -> Vec<(String, String)> {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let mut parsed = Vec::new();
  for line in reader.lines() {
    let line = line.expect("could not parse line");
    let mut iter = line.split_whitespace();
    let first = iter.next().expect("could not parse first choice").to_string();
    let second = iter.next().expect("could not parse second choice").to_string();
    parsed.push((first, second));
  }

  parsed
}

pub fn solve_first(filename: impl AsRef<Path>) -> u32 {
  let parsed_games = parse(filename);
  let mut games = Vec::new();

  for parsed_game in parsed_games {
    let (first, second) = parsed_game;
    let first_choice = match first.as_str() {
      "A" => Choice::Rock,
      "B" => Choice::Paper,
      "C" => Choice::Scissors,
      _ => panic!("invalid choice"),
    };
    let second_choice = match second.as_str() {
      "X" => Choice::Rock,
      "Y" => Choice::Paper,
      "Z" => Choice::Scissors,
      _ => panic!("invalid choice"),
    };

    games.push(Game((first_choice, second_choice)));
  }

  games.iter().map(|game| game.get_score()).sum()
}

pub fn solve_second(filename: impl AsRef<Path>) -> u32 {
  let parsed_games = parse(filename);
  let mut games = Vec::new();

  for parsed_game in parsed_games {
    let (first, second) = parsed_game;
    let choice = match first.as_str() {
      "A" => Choice::Rock,
      "B" => Choice::Paper,
      "C" => Choice::Scissors,
      _ => panic!("invalid choice"),
    };
    let strategy = match second.as_str() {
      "X" => Strategy::Lose,
      "Y" => Strategy::Draw,
      "Z" => Strategy::Win,
      _ => panic!("invalid strategy"),
    };

    let game = match (choice, strategy) {
      (Choice::Rock, Strategy::Lose) => Game((Choice::Rock, Choice::Scissors)),
      (Choice::Rock, Strategy::Draw) => Game((Choice::Rock, Choice::Rock)),
      (Choice::Rock, Strategy::Win) => Game((Choice::Rock, Choice::Paper)),
      (Choice::Paper, Strategy::Lose) => Game((Choice::Paper, Choice::Rock)),
      (Choice::Paper, Strategy::Draw) => Game((Choice::Paper, Choice::Paper)),
      (Choice::Paper, Strategy::Win) => Game((Choice::Paper, Choice::Scissors)),
      (Choice::Scissors, Strategy::Lose) => Game((Choice::Scissors, Choice::Paper)),
      (Choice::Scissors, Strategy::Draw) => Game((Choice::Scissors, Choice::Scissors)),
      (Choice::Scissors, Strategy::Win) => Game((Choice::Scissors, Choice::Rock)),
    };

    games.push(game);
  }

  games.iter().map(|game| game.get_score()).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_score_loss() {
    let game = Game((Choice::Paper, Choice::Rock));
    assert_eq!(game.get_score(), 1);
  }

  #[test]
  fn get_score_draw() {
    let game = Game((Choice::Scissors, Choice::Scissors));
    assert_eq!(game.get_score(), 6);
  }

  #[test]
  fn get_score_win() {
    let game = Game((Choice::Rock, Choice::Paper));
    assert_eq!(game.get_score(), 8);
  }

  #[test]
  fn solve_first_example() {
    let result = solve_first("example.txt");
    assert_eq!(result, 15);
  }

  #[test]
  fn solve_first_input() {
    let result = solve_first("input.txt");
    assert_eq!(result, 13924);
  }

  #[test]
  fn solve_second_example() {
    let result = solve_second("example.txt");
    assert_eq!(result, 12);
  }

  #[test]
  fn solve_second_input() {
    let result = solve_second("input.txt");
    assert_eq!(result, 13448);
  }
}