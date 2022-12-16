#![feature(iter_next_chunk)]

use aoc::{main, sample};

main!(d04, "Camp Cleanup");

fn d04(input: &str) -> (usize, usize) {
    let pairs: Vec<[u32; 4]> = input.lines()
        .filter_map(|line| line.split(&['-', ','])
            .filter_map(|n| n.parse().ok())
            .next_chunk().ok())
        .collect();
    let part1 = pairs.iter().filter(|[a1, a2, b1, b2]| a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2).count();
    let part2 = pairs.iter().filter(|[a1, a2, b1, b2]| b2 >= a1 && b1 <= a2).count();
    (part1, part2)
}

sample!(d04, 2, 4, "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
");
