#![feature(let_chains)]

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    x: usize,
    y: usize
}

impl Node {
    fn left(&self) -> Option<Self> {
        if self.x > 0 {
            Some(Self { x: self.x - 1, y: self.y })
        } else {
            None
        }
    }

    fn right(&self, width: usize) -> Option<Self> {
        if self.x < width - 1 {
            Some(Self { x: self.x + 1, y: self.y })
        } else {
            None
        }
    }

    fn up(&self) -> Option<Self> {
        if self.y > 0 {
            Some(Self { x: self.x, y: self.y - 1 })
        } else {
            None
        }
    }

    fn down(&self, height: usize) -> Option<Self> {
        if self.y < height - 1 {
            Some(Self { x: self.x, y: self.y + 1 })
        } else {
            None
        }
    }
}

#[derive(Debug, Eq)]
struct Candidate {
    distance: u32,
    node: Node
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

fn main() {
    let input: Vec<Vec<char>> = std::io::stdin().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut heights: HashMap<Node, i32> = HashMap::new();
    let mut end = Node { x: 0, y: 0 };
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            heights.insert(Node { x, y }, match input[y][x] {
                'a'..='z' => input[y][x] as i32 - 'a' as i32,
                'S' => 0,
                'E' => 25,
                _ => unreachable!()
            });
            if input[y][x] == 'E' {
                end = Node { x, y };
            }
        }
    }
    let mut distances: HashMap<Node, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Candidate { distance: 0, node: end });
    while let Some(candidate) = heap.pop() {
        if distances.contains_key(&candidate.node) {
            continue;
        }
        distances.insert(candidate.node.clone(), candidate.distance);
        if *heights.get(&candidate.node).unwrap() == 0 {
            println!("{}", candidate.distance);
            break;
        }
        if let Some(node) = candidate.node.left() && !distances.contains_key(&node) && height_diff(&heights, &candidate.node, &node) >= -1 {
            heap.push(Candidate { distance: candidate.distance + 1, node });
        }
        if let Some(node) = candidate.node.right(input[0].len()) && !distances.contains_key(&node) && height_diff(&heights, &candidate.node, &node) >= -1 {
            heap.push(Candidate { distance: candidate.distance + 1, node });
        }
        if let Some(node) = candidate.node.up() && !distances.contains_key(&node) && height_diff(&heights, &candidate.node, &node) >= -1 {
            heap.push(Candidate { distance: candidate.distance + 1, node });
        }
        if let Some(node) = candidate.node.down(input.len()) && !distances.contains_key(&node) && height_diff(&heights, &candidate.node, &node) >= -1 {
            heap.push(Candidate { distance: candidate.distance + 1, node });
        }
    }
}

fn height_diff(heights: &HashMap<Node, i32>, a: &Node, b: &Node) -> i32 {
    heights.get(b).unwrap() - heights.get(a).unwrap()
}
