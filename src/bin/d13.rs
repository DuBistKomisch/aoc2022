#![feature(array_chunks)]

use aoc::{main, sample};
use std::cmp::Ordering;

main!(d13, "Distress Signal");

#[derive(Clone, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32)
}

impl Packet {
    fn parse(text: &str) -> Self {
        Self::parse_recurse(text).0
    }

    fn parse_recurse(text: &str) -> (Self, &str) {
        if let Some(text) = text.strip_prefix('[') {
            if let Some(text) = text.strip_prefix(']') {
                (Self::List(vec![]), text)
            } else {
                let (mut item, mut text) = Self::parse_recurse(text);
                let mut items = vec![item];
                while let Some(next) = text.strip_prefix(',') {
                    (item, text) = Self::parse_recurse(next);
                    items.push(item);
                }
                (Self::List(items), &text[1..])
            }
        } else {
            let boundary = text.find(|c: char| !c.is_numeric()).unwrap();
            (Self::Integer(text[..boundary].parse().unwrap()), &text[boundary..])
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match [self, other] {
            [Self::Integer(a), Self::Integer(b)] => a.cmp(b),
            [Self::Integer(a), Self::List(b)] => vec![Self::Integer(*a)].cmp(b),
            [Self::List(a), Self::Integer(b)] => a.cmp(&vec![Self::Integer(*b)]),
            [Self::List(a), Self::List(b)] => a.cmp(b)
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn d13(input: &str) -> (usize, usize) {
    let mut packets: Vec<Packet> = input.lines()
        .filter_map(|line| (!line.is_empty()).then(|| Packet::parse(line)))
        .collect();

    let part1 = packets.array_chunks().enumerate()
        .filter_map(|(i, [left, right])| (left < right).then_some(i + 1))
        .sum();

    let dividers = [Packet::parse("[[2]]"), Packet::parse("[[6]]")];
    for divider in dividers.iter() {
        packets.push(divider.clone());
    }
    packets.sort();
    let part2 = dividers.iter()
        .filter_map(|divider| packets.iter().position(|packet| packet == divider).map(|i| i + 1))
        .product();

    (part1, part2)
}

sample!(d13, 13, 140, "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
");
