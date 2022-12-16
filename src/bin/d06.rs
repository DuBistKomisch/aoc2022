use aoc::{main, sample};

main!(d06, "Tuning Trouble");

fn d06(input: &str) -> (usize, usize) {
    (solve(input, 4), solve(input, 14))
}

fn solve(input: &str, marker: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    chars.windows(marker).position(|window| {
        let mut vec = window.to_vec();
        vec.sort();
        vec.dedup();
        vec.len() == marker
    }).unwrap() + marker
}

sample!(d06, 7, 19, "\
mjqjpqmgbljsphdztnvjfqwrcgsmlb
");
