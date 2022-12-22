use aoc::{main, sample};
use std::collections::VecDeque;

main!(d20, "Grove Positioning System");

const DECRYPTION_KEY: i64 = 811589153;

fn d20(input: &str) -> (i64, i64) {
    let input: Vec<i64> = input.lines().filter_map(|n| n.parse().ok()).collect();

    let mut part1 = input.iter().copied().enumerate().collect();
    mix(&mut part1);

    let mut part2 = input.iter().copied().map(|x| x * DECRYPTION_KEY).enumerate().collect();
    for _ in 0..10 {
        mix(&mut part2);
    }

    (coordinate(&part1), coordinate(&part2))
}

fn mix(buffer: &mut VecDeque<(usize, i64)>) {
    for i in 0..buffer.len() {
        let position = buffer.iter().position(|&(j, _)| i == j).unwrap();
        buffer.rotate_left(position);
        let old = buffer.pop_front().unwrap();
        if old.1 > 0 {
            buffer.rotate_left(old.1 as usize % buffer.len());
        } else {
            buffer.rotate_right(old.1.abs() as usize % buffer.len());
        }
        buffer.push_front(old);
    }
}

fn coordinate(buffer: &VecDeque<(usize, i64)>) -> i64 {
    let zero = buffer.iter().position(|&(_, x)| x == 0).unwrap();
    [1000, 2000, 3000].iter()
        .map(|&offset| buffer.iter().cycle().skip(zero).nth(offset).unwrap().1)
        .sum()
}

sample!(d20, 3, 1623178306, "\
1
2
-3
3
-2
0
4
");
