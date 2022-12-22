#![feature(iter_next_chunk)]

use aoc::{main, sample};
use std::collections::{HashMap, VecDeque};

main!(d21, "Monkey Math");

enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Op {
    fn parse(input: &str) -> Self {
        match input {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => unreachable!()
        }
    }

    fn calculate(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => a / b
        }
    }

    fn reverse_a(&self, b: u64, c: u64) -> u64 {
        match self {
            Self::Add => c - b,
            Self::Subtract => c + b,
            Self::Multiply => c / b,
            Self::Divide => c * b
        }
    }

    fn reverse_b(&self, a: u64, c: u64) -> u64 {
        match self {
            Self::Add => c - a,
            Self::Subtract => a - c,
            Self::Multiply => c / a,
            Self::Divide => a * c
        }
    }
}

struct Formula<'a> {
    op: Op,
    a: &'a str,
    b: &'a str
}

fn d21(input: &str) -> (u64, u64) {
    let mut numbers = HashMap::new();
    let mut formulas = HashMap::new();
    let mut index = HashMap::new();
    for line in input.lines() {
        let [left, right] = line.split(": ").next_chunk().unwrap();
        if let Ok(number) = right.parse() {
            numbers.insert(left, number);
        } else {
            let [a, op, b] = right.split_whitespace().next_chunk().unwrap();
            let op = Op::parse(op);
            index.entry(a).or_insert_with(|| vec![]).push(left);
            index.entry(b).or_insert_with(|| vec![]).push(left);
            formulas.insert(left, Formula { a, op, b });
        }
    }

    let mut part1 = numbers.clone();
    calculate(&mut part1, &formulas, &index);

    let mut part2 = numbers.clone();
    part2.remove(&"humn");
    calculate(&mut part2, &formulas, &index);
    reverse(&mut part2, &formulas);

    (part1[&"root"], part2[&"humn"])
}

fn calculate<'a>(numbers: &mut HashMap<&'a str, u64>, formulas: &HashMap<&'a str, Formula<'a>>, index: &HashMap<&'a str, Vec<&'a str>>) {
    let mut queue: VecDeque<_> = numbers.keys().copied().collect();
    while let Some(known) = queue.pop_front() {
        if !index.contains_key(known) {
            continue;
        }
        for next in &index[known] {
            let formula = &formulas[next];
            if !numbers.contains_key(next) && numbers.contains_key(formula.a) && numbers.contains_key(formula.b) {
                let a = numbers[formula.a];
                let b = numbers[formula.b];
                let result = formula.op.calculate(a, b);
                numbers.insert(next, result);
                queue.push_back(next);
            }
        }
    }
}

fn reverse<'a>(numbers: &mut HashMap<&'a str, u64>, formulas: &HashMap<&'a str, Formula<'a>>) {
    let mut monkey = "root";
    // special case: copy between root's children
    let formula = &formulas[monkey];
    if numbers.contains_key(formula.a) {
        numbers.insert(formula.b, numbers[formula.a]);
        monkey = formula.b;
    } else {
        numbers.insert(formula.a, numbers[formula.b]);
        monkey = formula.a;
    }
    while let Some(formula) = formulas.get(monkey) {
        if numbers.contains_key(formula.a) {
            let result = formula.op.reverse_b(numbers[formula.a], numbers[monkey]);
            numbers.insert(formula.b, result);
            monkey = formula.b;
        } else {
            let result = formula.op.reverse_a(numbers[formula.b], numbers[monkey]);
            numbers.insert(formula.a, result);
            monkey = formula.a;
        }
    }
}

sample!(d21, 152, 301, "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
");
