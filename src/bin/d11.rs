#![feature(iter_array_chunks)]

use aoc::{main, sample};
use std::collections::{VecDeque};

main!(d11, "Monkey in the Middle");

#[derive(Clone)]
enum Op {
    Square,
    Plus(u64),
    Times(u64)
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    div: u64,
    yes: usize,
    no: usize,
    inspections: u64
}

impl Monkey {
    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    fn inspect_item(&mut self) -> u64 {
        self.inspections += 1;
        let item = self.items.pop_front().unwrap();
        match self.op {
            Op::Square => item * item,
            Op::Plus(x) => item + x,
            Op::Times(x) => item * x
        }
    }

    fn throw_item(&self, item: u64) -> usize {
        if item % self.div == 0 {
            self.yes
        } else {
            self.no
        }
    }

    fn catch_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn d11(input: &str) -> (u64, u64) {
    let monkeys = parse(input);
    let multiple: u64 = monkeys.iter().map(|monkey| monkey.div).product();
    (
        solve(monkeys.clone(), 20, |item| item / 3),
        solve(monkeys.clone(), 10000, |item| item % multiple)
    )
}

fn parse(input: &str) -> Vec<Monkey> {
    input.lines()
        .filter(|line| !line.is_empty())
        .array_chunks()
        .map(|[_id, items, op, div, yes, no]| {
            let items: VecDeque<u64> = items
                .strip_prefix("  Starting items: ").unwrap()
                .split(", ").filter_map(|n| n.parse().ok())
                .collect();
            let op_parts: Vec<&str> = op
                .strip_prefix("  Operation: new = old ").unwrap()
                .split_whitespace().collect();
            let op = match &op_parts[..] {
                ["*", "old"] => Op::Square,
                ["+", x] => Op::Plus(x.parse().unwrap()),
                ["*", x] => Op::Times(x.parse().unwrap()),
                _ => unreachable!()
            };
            let div = div
                .strip_prefix("  Test: divisible by ").unwrap()
                .parse().unwrap();
            let yes = yes
                .strip_prefix("    If true: throw to monkey ").unwrap()
                .parse().unwrap();
            let no = no
                .strip_prefix("    If false: throw to monkey ").unwrap()
                .parse().unwrap();
            Monkey { items, op, div, yes, no, inspections: 0 }
        })
        .collect()
}

fn solve<F>(mut monkeys: Vec<Monkey>, iterations: usize, reduce: F) -> u64
where
    F: Fn(u64) -> u64
{
    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            while monkeys[i].has_items() {
                let item = monkeys[i].inspect_item();
                let item = reduce(item);
                let target = monkeys[i].throw_item(item);
                monkeys[target].catch_item(item);
            }
        }
    }
    let mut inspections: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    inspections.sort();
    inspections.reverse();
    inspections.iter().take(2).product()
}

sample!(d11, 10605, 2713310158, "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
");
