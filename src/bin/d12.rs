#![feature(let_chains)]

use aoc::{main, sample};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};

main!(d12, "Hill Climbing Algorithm");

#[derive(Clone, Eq, Hash, PartialEq)]
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

#[derive(Eq)]
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

fn d12(input: &str) -> (u32, u32) {
    let input: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut heights: HashMap<Node, i32> = HashMap::new();
    let mut start = Node { x: 0, y: 0 };
    let mut end = Node { x: 0, y: 0 };
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            heights.insert(Node { x, y }, match input[y][x] {
                'a'..='z' => input[y][x] as i32 - 'a' as i32,
                'S' => 0,
                'E' => 25,
                _ => unreachable!()
            });
            if input[y][x] == 'S' {
                start = Node { x, y };
            }
            if input[y][x] == 'E' {
                end = Node { x, y };
            }
        }
    }
    (
        bfs(
            input[0].len(),
            input.len(),
            start,
            |node| *node == end,
            |from, to| heights.get(to).unwrap() - heights.get(from).unwrap() <= 1
        ),
        bfs(
            input[0].len(),
            input.len(),
            end,
            |node| *heights.get(node).unwrap() == 0,
            |from, to| heights.get(to).unwrap() - heights.get(from).unwrap() >= -1
        )
    )
}

fn bfs<F, E>(width: usize, height: usize, start: Node, finish: F, edge: E) -> u32
where
    F: Fn(&Node) -> bool,
    E: Fn(&Node, &Node) -> bool
{
    let mut distances: HashMap<Node, u32> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(Candidate { distance: 0, node: start });
    while let Some(candidate) = queue.pop_front() {
        if distances.contains_key(&candidate.node) {
            continue;
        }
        distances.insert(candidate.node.clone(), candidate.distance);
        if finish(&candidate.node) {
            return candidate.distance;
        }
        let distance = candidate.distance + 1;
        if let Some(node) = candidate.node.left() && !distances.contains_key(&node) && edge(&candidate.node, &node) {
            queue.push_back(Candidate { distance, node });
        }
        if let Some(node) = candidate.node.right(width) && !distances.contains_key(&node) && edge(&candidate.node, &node) {
            queue.push_back(Candidate { distance, node });
        }
        if let Some(node) = candidate.node.up() && !distances.contains_key(&node) && edge(&candidate.node, &node) {
            queue.push_back(Candidate { distance, node });
        }
        if let Some(node) = candidate.node.down(height) && !distances.contains_key(&node) && edge(&candidate.node, &node) {
            queue.push_back(Candidate { distance, node });
        }
    }
    unreachable!()
}

sample!(d12, 31, 29, "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
");
