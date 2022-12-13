use std::cmp::Ordering;

#[derive(Clone, Debug, Eq)]
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
    let mut packets: Vec<Packet> = std::io::stdin().lines()
        .filter_map(|line| line.ok()
            .and_then(|line| if line.is_empty() { None } else { Some(Packet::parse(&line)) }))
        .collect();
    let dividers = [Packet::parse("[[2]]"), Packet::parse("[[6]]")];
    for divider in dividers.iter() {
        packets.push(divider.clone());
    }
    packets.sort();
    let answer: usize = dividers.iter()
        .map(|divider| packets.iter().position(|packet| packet == divider).unwrap() + 1)
        .product();
    println!("{}", answer);
}
