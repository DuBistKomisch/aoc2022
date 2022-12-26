use aoc::{main, sample};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

main!(d23, "Unstable Diffusion");

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

const DIRECTIONS: [Point; 8] = [
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: -1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 }
];

// N S W E
const PROPOSALS: [i32; 4] = [6, 2, 4, 0];

fn d23(input: &str) -> (i32, i32) {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if c == '#' {
                elves.insert(Point { x, y });
            }
        }
    }

    let mut elves1 = elves.clone();
    let mut proposal1 = 0;
    for _ in 0..10 {
        run(&mut elves1, proposal1);
        proposal1 = (proposal1 + 1).rem_euclid(PROPOSALS.len() as i32);
    }

    let mut elves2 = elves.clone();
    let mut proposal2 = 0;
    let mut round2 = 1;
    loop {
        let moved = run(&mut elves2, proposal2);
        if moved == 0 {
            break;
        }
        proposal2 = (proposal2 + 1).rem_euclid(PROPOSALS.len() as i32);
        round2 += 1;
    }

    (space(&elves1), round2)
}

fn run(elves: &mut HashSet<Point>, proposal: i32) -> usize {
    let mut elf_to_move = HashMap::new();
    let mut move_to_elf = HashMap::new();
    'elf: for elf in elves.iter() {
        if (0..DIRECTIONS.len()).any(|dir| elves.contains(&(*elf + DIRECTIONS[dir as usize]))) {
            for check in 0..PROPOSALS.len() {
                let dir = PROPOSALS[(proposal + check as i32).rem_euclid(PROPOSALS.len() as i32) as usize];
                if [-1, 0, 1].iter().any(|angle| {
                    let direction = DIRECTIONS[(dir + angle).rem_euclid(DIRECTIONS.len() as i32) as usize];
                    let next = *elf + direction;
                    elves.contains(&next)
                }) {
                    continue;
                }
                let direction = DIRECTIONS[dir as usize];
                let next = *elf + direction;
                if move_to_elf.contains_key(&next) {
                    let other = move_to_elf[&next];
                    elf_to_move.insert(other, other);
                    elf_to_move.insert(*elf, *elf);
                } else {
                    elf_to_move.insert(*elf, next);
                    move_to_elf.insert(next, *elf);
                }
                continue 'elf;
            }
        }
        elf_to_move.insert(*elf, *elf);
    }
    let moved = elf_to_move.iter().filter(|(elf, next)| elf != next).count();
    *elves = elf_to_move.into_values().collect();
    moved
}

fn space(elves: &HashSet<Point>) -> i32 {
    let xs = elves.iter().map(|Point { x, .. }| x);
    let ys = elves.iter().map(|Point { y, .. }| y);
    let width = xs.clone().max().unwrap() - xs.min().unwrap() + 1;
    let height = ys.clone().max().unwrap() - ys.min().unwrap() + 1;
    width * height - elves.len() as i32
}

sample!(d23, 110, 20, "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
");
