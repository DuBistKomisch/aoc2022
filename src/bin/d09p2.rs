#![feature(array_windows)]

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let mut knots = vec![Point { x: 0, y: 0 }; 10];
    let mut seen = HashSet::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let [direction, times] = &line.split_whitespace().collect::<Vec<&str>>()[..] else { unreachable!() };
        for _ in 0..times.parse().unwrap() {
            let mut head = &mut knots[0];
            match *direction {
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
            seen.insert(knots.last().unwrap().clone());
        }
    }
    println!("{}", seen.len());
}
