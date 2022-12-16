#![feature(iter_next_chunk)]

use aoc::{main, sample};
use std::collections::HashSet;

main!(d09, "Rope Bridge");

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

fn d09(input: &str) -> (usize, usize) {
    (solve(input, 2), solve(input, 10))
}

fn solve(input: &str, knots: usize) -> usize {
    let mut knots = vec![Point { x: 0, y: 0 }; knots];
    let mut seen = HashSet::new();
    for line in input.lines() {
        let [direction, times] = line.split_whitespace().next_chunk().unwrap();
        for _ in 0..times.parse().unwrap() {
            let mut head = &mut knots[0];
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => unreachable!()
            };
            for i in 0..knots.len() - 1 {
                let head = knots[i];
                let mut tail = &mut knots[i + 1];
                if head.x == tail.x {
                    if (head.y - tail.y).abs() == 2 {
                        tail.y += (head.y - tail.y).signum();
                    }
                } else if head.y == tail.y {
                    if (head.x - tail.x).abs() == 2 {
                        tail.x += (head.x - tail.x).signum();
                    }
                } else if (head.x - tail.x).abs() == 2 || (head.y - tail.y).abs() == 2 {
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                }
            }
            seen.insert(knots[knots.len() - 1].clone());
        }
    }
    seen.len()
}

sample!(d09, 13, 1, "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
");
