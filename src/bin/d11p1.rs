use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
enum Op {
    Square,
    Plus(u32),
    Times(u32)
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    op: Op,
    div: u32,
    yes: u32,
    no: u32,
    inspections: u32
}

impl Monkey {
    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    fn inspect_item(&mut self) -> (u32, u32) {
        let mut worry = self.items.pop_front().unwrap();
        worry = match self.op {
            Op::Square => worry * worry,
            Op::Plus(x) => worry + x,
            Op::Times(x) => worry * x
        };
        worry /= 3;
        let target = if worry % self.div == 0 { self.yes } else { self.no };
        self.inspections += 1;
        (target, worry)
    }

    fn catch_item(&mut self, item: u32) {
        self.items.push_back(item);
    }
}

fn main() {
    let mut monkeys = BTreeMap::new();
    let mut lines = std::io::stdin().lines();
    while let Some(id) = lines.next() {
        let id: u32 = id.unwrap()
            .strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap()
            .parse().unwrap();
        let items: VecDeque<u32> = lines.next().unwrap().unwrap()
            .strip_prefix("  Starting items: ").unwrap()
            .split(", ").filter_map(|n| n.parse().ok()).collect();
        let ops_line = lines.next().unwrap().unwrap();
        let ops_parts: Vec<&str> = ops_line
            .strip_prefix("  Operation: new = old ").unwrap()
            .split_whitespace().collect();
        let op = match &ops_parts[..] {
            ["*", "old"] => Op::Square,
            ["+", x] => Op::Plus(x.parse().unwrap()),
            ["*", x] => Op::Times(x.parse().unwrap()),
            _ => unreachable!()
        };
        let div: u32 = lines.next().unwrap().unwrap()
            .strip_prefix("  Test: divisible by ").unwrap()
            .parse().unwrap();
        let yes: u32 = lines.next().unwrap().unwrap()
            .strip_prefix("    If true: throw to monkey ").unwrap()
            .parse().unwrap();
        let no: u32 = lines.next().unwrap().unwrap()
            .strip_prefix("    If false: throw to monkey ").unwrap()
            .parse().unwrap();
        lines.next();
        let monkey = Monkey { items, op, div, yes, no, inspections: 0 };
        monkeys.insert(id, RefCell::new(monkey));
    }
    for _ in 0..20 {
        for monkey in monkeys.values() {
            while monkey.borrow().has_items() {
                let (target, item) = monkey.borrow_mut().inspect_item();
                monkeys.get(&target).unwrap().borrow_mut().catch_item(item);
            }
        }
    }
    let mut inspections: Vec<u32> = monkeys.values().map(|monkey| monkey.borrow().inspections).collect();
    inspections.sort();
    let answer: u32 = inspections.iter().rev().take(2).product();
    println!("{}", answer);
}
