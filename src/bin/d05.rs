#![feature(iter_next_chunk)]

use aoc::{main, sample};

main!(d05, "Supply Stacks");

fn d05(input: &str) -> (String, String) {
    (solve(input, true), solve(input, false))
}

fn solve(input: &str, reverse: bool) -> String {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut lines = input.lines();
    'crates: for line in lines.by_ref() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_numeric() {
                break 'crates;
            }
            if c.is_whitespace() {
                continue;
            }
            stacks[i].insert(0, c);
        }
    }
    lines.next();
    for line in lines {
        let [n, from, to]: [usize; 3] = line.split(' ')
            .filter_map(|n| n.parse().ok())
            .next_chunk().unwrap();
        let at = stacks[from - 1].len() - n;
        let mut moved = stacks[from - 1].split_off(at);
        if reverse {
            moved.reverse();
        }
        stacks[to - 1].append(&mut moved);
    }
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

sample!(d05, "CMZ", "MCD", r"\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
");
