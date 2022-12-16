#![feature(array_windows)]
#![feature(iter_next_chunk)]
#![feature(let_chains)]

use aoc::{main, sample};
use std::collections::HashSet;

main!(d14, "Regolith Reservoir");

fn d14(input: &str) -> (u32, u32) {
    let mut filled = HashSet::new();
    for line in input.lines() {
        let coords: Vec<[i32; 2]> = line.split(" -> ")
            .map(|coord| coord.split(',')
                .filter_map(|n| n.parse().ok())
                .next_chunk().unwrap())
            .collect();
        for [a, b] in coords.array_windows() {
            if a[0] == b[0] {
                let mut y = a[1];
                while y != b[1] {
                    filled.insert((a[0], y));
                    y += (b[1] - a[1]).signum();
                }
            } else if a[1] == b[1] {
                let mut x = a[0];
                while x != b[0] {
                    filled.insert((x, a[1]));
                    x += (b[0] - a[0]).signum();
                }
            }
        }
        let last = coords[coords.len() - 1];
        filled.insert((last[0], last[1]));
    }
    let bedrock = *filled.iter().map(|(_x, y)| y).max().unwrap();
    let mut part2 = 0;
    let mut part1 = 0;
    loop {
        let mut sand = (500, 0);
        'drop: loop {
            if sand.1 > bedrock {
                if part1 == 0 {
                    part1 = part2;
                }
                break;
            }
            for delta in [(0, 1), (-1, 1), (1, 1)] {
                let next = (sand.0 + delta.0, sand.1 + delta.1);
                if !filled.contains(&next) {
                    sand = next;
                    continue 'drop;
                }
            }
            break;
        }
        filled.insert(sand);
        part2 += 1;
        if sand.1 == 0 {
            break;
        }
    }
    (part1, part2)
}

sample!(d14, 24, 93, "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
");
