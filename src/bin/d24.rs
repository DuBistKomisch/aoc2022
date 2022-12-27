use aoc::{main, sample};
use std::collections::HashSet;
use std::ops::{Add, AddAssign};

main!(d24, "Blizzard Basin");

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 }
];

fn d24(input: &str) -> (i32, i32) {
    let mut blizzards = vec![];
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if let Some(dir) = match c {
                '>' => Some(0),
                'v' => Some(1),
                '<' => Some(2),
                '^' => Some(3),
                _ => None
            } {
                blizzards.push((Point { x: x - 1, y: y - 1 }, dir));
            }
        }
    }
    let width = input.lines().nth(0).unwrap().chars().count() as i32 - 2;
    let height = input.lines().count() as i32 - 2;
    let start = Point {
        x: input.lines()
            .nth(0).unwrap().chars()
            .position(|c| c == '.').unwrap() as i32 - 1,
        y: -1
    };
    let end = Point {
        x: input.lines()
            .nth(height as usize + 1).unwrap().chars()
            .position(|c| c == '.').unwrap() as i32 - 1,
        y: height
    };
    let there = travel(&mut blizzards, &start, &end, width, height);
    let back = travel(&mut blizzards, &end, &start, width, height);
    let again = travel(&mut blizzards, &start, &end, width, height);
    (there, there + back + again)
}

fn travel(blizzards: &mut Vec<(Point, i32)>, from: &Point, to: &Point, width: i32, height: i32) -> i32 {
    let mut minutes = 0;
    let mut positions = HashSet::new();
    positions.insert(from.clone());
    while !positions.contains(to) {
        step(blizzards, width, height);
        let blizzard_positions: HashSet<_> = blizzards.iter()
            .map(|(point, _dir)| point)
            .copied().collect();
        let mut next_positions = HashSet::new();
        for position in positions {
            for dir in 0..DIRECTIONS.len() {
                let next = position + DIRECTIONS[dir];
                if next != *from && next != *to && (
                    blizzard_positions.contains(&next)
                    || next.x < 0 || next.x >= width
                    || next.y < 0 || next.y >= height
                ) {
                    continue;
                }
                next_positions.insert(next);
            }
            if !blizzard_positions.contains(&position) {
                next_positions.insert(position);
            }
        }
        positions = next_positions;
        minutes += 1;
    }
    minutes
}

fn step(blizzards: &mut Vec<(Point, i32)>, width: i32, height: i32) {
    *blizzards = blizzards.into_iter()
        .map(|&mut (point, dir)| {
            let mut next = point + DIRECTIONS[dir as usize];
            if next.x < 0 {
                next.x = width - 1;
            }
            if next.x >= width {
                next.x = 0;
            }
            if next.y < 0 {
                next.y = height - 1;
            }
            if next.y >= height {
                next.y = 0;
            }
            (next, dir)
        })
        .collect()
}

sample!(d24, 18, 54, "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
");
