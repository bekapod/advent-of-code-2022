#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};
use core::fmt::Debug;

#[derive(Debug, Clone)]
struct MonkeyTest {
  divisible_by: u64,
  if_true: usize,
  if_false: usize,
}

#[derive(Debug, Clone)]
enum Operation {
  Add,
  Subtract,
  Multiply,
  Divide,
}

#[derive(Debug, Clone)]
struct MonkeyOperation {
  operator: Operation,
  value: Option<u64>,
}

#[derive(Debug, Clone)]
struct Monkey {
  items: Vec<u64>,
  operation: MonkeyOperation,
  test: MonkeyTest,
  inspected_item_count: usize,
}
impl Monkey {
  fn new(starting_items: Vec<u64>, operation: MonkeyOperation, test: MonkeyTest) -> Monkey {
    Monkey {
      items: starting_items,
      operation,
      test,
      inspected_item_count: 0,
    }
  }

  fn operate_on_item(&self, item: &u64) -> u64 {
    let value = self.operation.value.unwrap_or(*item);
    match self.operation.operator {
      Operation::Add => item + value,
      Operation::Subtract => item - value,
      Operation::Multiply => item * value,
      Operation::Divide => item / value,
    }
  }

  fn test_item(&self, item: u64) -> bool {
    item % self.test.divisible_by == 0
  }
}

struct MonkeyGang {
  is_calming_down: bool,
  monkeys: Vec<Monkey>,
}
impl MonkeyGang {
  fn play_round(&mut self, idx: usize) {
    let monkeys = &mut self.monkeys;
    let items = std::mem::take(&mut monkeys[idx].items);
    let chinese_remainder_theorem_idk: u64 = monkeys.iter().map(|m| m.test.divisible_by).product();
    monkeys[idx].inspected_item_count += items.len();

    for item in items {
      let monkey = &mut monkeys[idx];
      let mut new_item = monkey.operate_on_item(&item) % chinese_remainder_theorem_idk;

      if self.is_calming_down {
        new_item = new_item / 3;
      }

      let monkey_to_throw_to = if monkey.test_item(new_item) {
        monkey.test.if_true
      } else {
        monkey.test.if_false
      };
      monkeys[monkey_to_throw_to].items.push(new_item);
    }
  }
}

fn parse(filename: impl AsRef<Path>) -> Vec<Monkey> {
  let file = File::open(filename).expect("file not found");
  let reader = BufReader::new(file);
  let all_lines = reader.lines().collect::<Vec<_>>();
  let groups = all_lines.chunks(7);

  let mut monkeys = vec![];

  for group in groups {
    let starting_items = group[1].as_ref().expect("no starting items line")
      .trim().split(": ").nth(1).expect("no starting items")
      .split(", ").map(|s| s.parse::<u64>().expect("not a number"))
      .collect::<Vec<_>>();

    let operation_line = group[2].as_ref().expect("no operation line")
        .trim().split("= ").nth(1).expect("no operation")
        .split(" ").collect::<Vec<_>>();
    let operation = MonkeyOperation {
      operator: match operation_line[1] {
        "+" => Operation::Add,
        "*" => Operation::Multiply,
        "-" => Operation::Subtract,
        "/" => Operation::Divide,
        _ => panic!("unknown operator"),
      },
      value: match operation_line[2] {
        "old" => None,
        other => Some(other.parse::<u64>().expect("not a number")),
      },
    };

    let divisor_line = group[3].as_ref().expect("no divisor line")
        .trim().split(" ").collect::<Vec<_>>();
    let true_line = group[4].as_ref().expect("no if true line")
        .trim().split(" ").collect::<Vec<_>>();
    let false_line = group[5].as_ref().expect("no if false line").trim()
        .split(" ").collect::<Vec<_>>();
    let test = MonkeyTest {
      divisible_by: divisor_line[3].parse::<u64>().expect("not a number"),
      if_true: true_line[5].parse::<usize>().expect("not a number"),
      if_false: false_line[5].parse::<usize>().expect("not a number"),
    };

    let monkey = Monkey::new(starting_items, operation, test);
    monkeys.push(monkey);
  }

  monkeys
}

fn play(monkeys: &Vec<Monkey>, rounds: u64, should_calm_down: bool) -> Vec<Monkey> {
  let mut gang = MonkeyGang { monkeys: monkeys.clone(), is_calming_down: should_calm_down };

  for _ in 0..rounds {
    for idx in 0..monkeys.len() {
      gang.play_round(idx);
    }
  }

  gang.monkeys
}

pub fn solve(filename: impl AsRef<Path>) -> (usize, usize) {
  let monkeys = parse(filename);
  let mut monkeys_after_20_rounds = play(&monkeys, 20, true);
  monkeys_after_20_rounds.sort_by(|a, b| a.inspected_item_count.cmp(&b.inspected_item_count));
  monkeys_after_20_rounds.reverse();

  let mut monkeys_after_10000_rounds = play(&monkeys, 10000, false);
  monkeys_after_10000_rounds.sort_by(|a, b| a.inspected_item_count.cmp(&b.inspected_item_count));
  monkeys_after_10000_rounds.reverse();

  (
    (0..2).map(|idx| monkeys_after_20_rounds[idx].inspected_item_count).product(),
    (0..2).map(|idx| monkeys_after_10000_rounds[idx].inspected_item_count).product(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_example() {
    assert_eq!(solve("example.txt"), (10605, 2713310158));
  }

  #[test]
  fn solve_input() {
    assert_eq!(solve("input.txt"), (111210, 15447387620));
  }
}