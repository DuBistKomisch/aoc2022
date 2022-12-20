#![feature(anonymous_lifetime_in_impl_trait)]

use aoc::{main, sample};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

main!(d17, "Pyroclastic Flow");

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

fn d17(input: &str) -> (i32, i64) {
    let jets: Vec<Point> = input.trim().chars()
        .map(|c| match c {
            '<' => Point { x: -1, y: 0 },
            '>' => Point { x: 1, y: 0 },
            _ => unreachable!()
        })
        .collect();
    let pieces: Vec<Vec<Point>> = vec![
        // ####
        vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 3, y: 0 }],
        // .#.
        // ###
        // .#.
        vec![Point { x: 1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }, Point { x: 1, y: 2 }],
        // ..#
        // ..#
        // ###
        vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 2, y: 1 }, Point { x: 2, y: 2 }],
        // #
        // #
        // #
        // #
        vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: 2 }, Point { x: 0, y: 3 }],
        // ##
        // ##
        vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }],
    ];

    let mut part1 = HashSet::new();
    let mut height1 = 0;
    let mut jets1 = jets.iter().cycle();
    for piece in pieces.iter().cycle().take(2022) {
        let mut position = Point { x: 2, y: height1 + 3 };
        loop {
            let jet = jets1.next().unwrap();
            if play(&part1, &mut position, piece, jet) {
                break;
            }
        }
        height1 = settle(&mut part1, position, piece, height1);
    }

    let mut part2: HashSet<Point> = HashSet::new();
    let mut height2 = 0;
    let mut piece = 0;
    let mut jet = 0;
    let mut tick = 0;
    let mut seen1 = HashMap::new();
    let mut seen2 = HashMap::new();
    let part2_start_height: i32;
    let part2_start_pieces: i64;
    let part2_cycle_height: i32;
    let part2_cycle_pieces: i64;
    loop {
        if seen2.contains_key(&(piece, jet)) {
            let (prev_tick, prev_height) = seen2[&(piece, jet)];
            let (first_tick, first_height) = seen1[&(piece, jet)];
            if tick - prev_tick == prev_tick - first_tick && height2 - prev_height == prev_height - first_height {
                part2_start_height = first_height;
                part2_start_pieces = first_tick;
                part2_cycle_height = prev_height - first_height;
                part2_cycle_pieces = prev_tick - first_tick;
                break;
            }
        } else if seen1.contains_key(&(piece, jet)) {
            seen2.insert((piece, jet), (tick, height2));
        } else {
            seen1.insert((piece, jet), (tick, height2));
        }
        let mut position = Point { x: 2, y: height2 + 3 };
        loop {
            let settled = play(&part2, &mut position, &pieces[piece], &jets[jet]);
            jet = (jet + 1) % jets.len();
            if settled {
                break;
            }
        }
        height2 = settle(&mut part2, position, &pieces[piece], height2);
        piece = (piece + 1) % pieces.len();
        tick += 1;
    }
    let cycles = (1_000_000_000_000 - part2_start_pieces) / part2_cycle_pieces;
    let part2_final_pieces = (1_000_000_000_000 - part2_start_pieces) % part2_cycle_pieces;
    loop {
        if tick == part2_start_pieces + part2_cycle_pieces * 2 + part2_final_pieces {
            break;
        }
        let mut position = Point { x: 2, y: height2 + 3 };
        loop {
            let settled = play(&part2, &mut position, &pieces[piece], &jets[jet]);
            jet = (jet + 1) % jets.len();
            if settled {
                break;
            }
        }
        height2 = settle(&mut part2, position, &pieces[piece], height2);
        piece = (piece + 1) % pieces.len();
        tick += 1;
    }
    let part2_final_height = height2 - part2_start_height - part2_cycle_height * 2;
    let part2_total_height = part2_start_height as i64 + part2_cycle_height as i64 * cycles + part2_final_height as i64;

    (
        height1,
        part2_total_height
    )
}

fn play(solid: &HashSet<Point>, position: &mut Point,
        piece: &Vec<Point>, jet: &Point) -> bool {
    let blocked = piece.iter().any(|part| {
        let new = *position + *jet + *part;
        new.x < 0 || new.x > 6 || solid.contains(&new)
    });
    if !blocked {
        *position += *jet;
    }
    let gravity = Point { x: 0, y: -1 };
    let blocked = piece.iter().any(|part| {
        let new = *position + gravity + *part;
        new.y < 0 || solid.contains(&new)
    });
    if !blocked {
        *position += gravity;
    }
    blocked
}

fn settle(solid: &mut HashSet<Point>, position: Point,
          piece: &Vec<Point>, mut height: i32) -> i32 {
    for part in piece.iter() {
        let new = position + *part;
        if new.y + 1 > height {
            height = new.y + 1;
        }
        solid.insert(new);
    }
    height
}

sample!(d17, 3068, 1514285714288, "\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
");
