#![feature(iter_next_chunk)]

use aoc::{main, sample};
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::ops::Add;

main!(d18, "Boiling Boulders");

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

fn d18(input: &str) -> (u32, u32) {
    let cubes: HashSet<Point> = input.lines()
        .map(|line| line.split(',')
            .filter_map(|n| n.parse().ok())
            .next_chunk().unwrap())
        .map(|[x, y, z]| Point { x, y, z })
        .collect();
    let directions: Vec<Point> = vec![
        Point { x: 1, y: 0, z: 0 },
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: 0, z: -1 },
    ];

    let mut surface = 0;
    for cube in cubes.iter() {
        for direction in directions.iter() {
            if !cubes.contains(&(*cube + *direction)) {
                surface += 1;
            }
        }
    }

    let low = cubes.iter()
        .map(|cube| min(cube.x, min(cube.y, cube.z)))
        .min().unwrap() - 1;
    let high = cubes.iter()
        .map(|cube| max(cube.x, max(cube.y, cube.z)))
        .max().unwrap() + 1;
    let mut air: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(Point { x: low, y: low, z: low });
    while let Some(point) = queue.pop_front() {
        if air.contains(&point) {
            continue;
        }
        air.insert(point);
        for direction in directions.iter() {
            let next = point + *direction;
            if air.contains(&next) || cubes.contains(&next)
                || next.x < low || next.y < low || next.z < low
                || next.x > high || next.y > high || next.z > high {
                continue;
            }
            queue.push_back(next);
        }
    }
    let mut external = 0;
    for cube in cubes.iter() {
        for direction in directions.iter() {
            if air.contains(&(*cube + *direction)) {
                external += 1;
            }
        }
    }

    (surface, external)
}

sample!(d18, 64, 58, "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
");
