#![feature(iter_array_chunks)]

use aoc::{main, sample};
use std::collections::HashSet;

main!(d03, "Rucksack Reorganization");

fn d03(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let compartment = line.chars().count() / 2;
        let seen: HashSet<char> = line.chars().take(compartment).collect();
        let shared = line.chars().skip(compartment).find(|c| seen.contains(c)).unwrap();
        priority(shared)
    }).sum()
}

fn part2(input: &str) -> u32 {
    input.lines().array_chunks().map(|[line1, line2, line3]| {
        let seen: HashSet<char> = line1.chars().collect();
        let twice: HashSet<char> = line2.chars().filter(|c| seen.contains(c)).collect();
        let badge = line3.chars().find(|c| twice.contains(c)).unwrap();
        priority(badge)
    }).sum()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0
    }
}

sample!(d03, 157, 70, "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
");
