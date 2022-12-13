#![feature(iter_next_chunk)]

use std::cmp::Ordering;

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32)
}

impl Packet {
    fn parse(text: &str) -> Self {
        Self::parse_recurse(text).0
    }

    fn parse_recurse(text: &str) -> (Self, &str) {
        if let Some(text) = text.strip_prefix("[") {
            if let Some(text) = text.strip_prefix("]") {
                (Self::List(vec![]), text)
            } else {
                let (mut item, mut text) = Self::parse_recurse(text);
                let mut items = vec![item];
                while let Some(next) = text.strip_prefix(",") {
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

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut index = 1;
    let mut answer = 0;
    while let Ok([left, right]) = lines.next_chunk() {
        let left = Packet::parse(&left.unwrap());
        let right = Packet::parse(&right.unwrap());
        if left < right {
            answer += index;
        }
        lines.next();
        index += 1;
    }
    println!("{}", answer);
}
