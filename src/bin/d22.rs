use aoc::{main, sample};
use std::collections::{HashMap, VecDeque};
use std::ops::{Add, AddAssign};

main!(d22, "Monkey Map", 50);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

enum Tile {
    Open,
    Wall
}

const FACES: [[i32; 4]; 6] = [
    [1, 2, 4, 3],
    [5, 2, 0, 3],
    [1, 5, 4, 0],
    [1, 0, 4, 5],
    [0, 2, 5, 3],
    [1, 3, 4, 2]
];

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 }
];

fn d22(input: &str, size: i32) -> (i32, i32) {
    let mut start = Point { x: -1, y: -1 };
    let mut lines = input.lines();
    let mut tiles = HashMap::new();
    let mut point_to_face = HashMap::new();
    let mut face_to_point = HashMap::new();
    let mut face_to_spin = HashMap::new();
    for (y, line) in lines.by_ref().enumerate() {
        let y = y as i32;
        if line.is_empty() {
            break;
        }
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if c == '.' {
                if start.x == -1 {
                    start = Point { x, y };
                }
                tiles.insert(Point { x, y }, Tile::Open);
            } else if c == '#' {
                tiles.insert(Point { x, y }, Tile::Wall);
            }
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back((0, 1, 0, Point { x: start.x / size, y: start.y / size }));
    while let Some((face, prev, from, point)) = queue.pop_front() {
        if !tiles.contains_key(&Point { x: point.x * size, y: point.y * size }) {
            continue;
        }
        if point_to_face.contains_key(&point) {
            continue;
        }
        let spin = modulo(from - FACES[face as usize].iter().position(|&adj| adj == prev).unwrap() as i32, 4);
        point_to_face.insert(point, face);
        face_to_point.insert(face, point);
        face_to_spin.insert(face, spin);
        for dir in 0..4 {
            queue.push_back((
                FACES[face as usize][modulo(dir - spin, 4) as usize],
                face,
                modulo(dir + 2, 4),
                point + DIRECTIONS[dir as usize]
            ));
        }
    }
    let line = lines.next().unwrap();
    let steps: Vec<_> = line.split(&['L', 'R'])
        .filter_map(|x| x.parse().ok())
        .collect();
    let turns: Vec<_> = line.split(char::is_numeric)
        .filter_map(|x| match x {
            "L" => Some(-1),
            "R" => Some(1),
            _ => None
        })
        .collect();

    let (pos1, dir1) = run(start.clone(), &steps, &turns, &tiles, |next, pos, dir, _| {
        match dir {
            0 => next.x = tiles.keys().filter_map(|Point { x, y }| (*y == pos.y).then_some(*x)).min().unwrap(),
            1 => next.y = tiles.keys().filter_map(|Point { x, y }| (*x == pos.x).then_some(*y)).min().unwrap(),
            2 => next.x = tiles.keys().filter_map(|Point { x, y }| (*y == pos.y).then_some(*x)).max().unwrap(),
            3 => next.y = tiles.keys().filter_map(|Point { x, y }| (*x == pos.x).then_some(*y)).max().unwrap(),
            _ => unreachable!()
        }
    });

    let (pos2, dir2) = run(start.clone(), &steps, &turns, &tiles, |next, pos, dir, next_dir| {
        let prev = point_to_face[&Point { x: pos.x / size, y: pos.y / size }];
        let face = FACES[prev as usize][modulo(dir - face_to_spin[&prev], 4) as usize];
        let spin = modulo(FACES[face as usize].iter().position(|&adj| adj == prev).unwrap() as i32 + face_to_spin[&face], 4);
        let to = face_to_point[&face];
        let offset = match dir {
            0 => pos.y - pos.y / size * size,
            1 => size - (pos.x - pos.x / size * size) - 1,
            2 => size - (pos.y - pos.y / size * size) - 1,
            3 => pos.x - pos.x / size * size,
            _ => unreachable!()
        };
        next.x = match spin {
            0 => to.x * size + size - 1,
            1 => to.x * size + offset,
            2 => to.x * size,
            3 => to.x * size + size - offset - 1,
            _ => unreachable!()
        };
        next.y = match spin {
            0 => to.y * size + size - offset - 1,
            1 => to.y * size + size - 1,
            2 => to.y * size + offset,
            3 => to.y * size,
            _ => unreachable!()
        };
        *next_dir = modulo(spin + 2, 4);
    });

    (password(pos1, dir1), password(pos2, dir2))
}

fn run<F>(mut pos: Point, steps: &Vec<u32>, turns: &Vec<i32>, tiles: &HashMap<Point, Tile>, wrap: F) -> (Point, i32)
where
    F: Fn(&mut Point, &Point, i32, &mut i32)
{
    let mut dir = 0;
    let mut steps = steps.iter().copied();
    let mut turns = turns.iter().copied();
    loop {
        let Some(step): Option<u32> = steps.next() else { break; };
        'step: for _ in 0..step {
            let mut next = pos + DIRECTIONS[dir as usize];
            let mut next_dir = dir;
            loop {
                match tiles.get(&next) {
                    Some(Tile::Open) => {
                        pos = next;
                        dir = next_dir;
                        break;
                    },
                    Some(Tile::Wall) => {
                        break 'step;
                    },
                    None => {
                        wrap(&mut next, &pos, dir, &mut next_dir);
                    }
                }
            }
        }
        let Some(turn): Option<i32> = turns.next() else { break; };
        dir = modulo(dir + turn, 4);
    }
    (pos, dir)
}

fn password(point: Point, direction: i32) -> i32 {
    (point.y + 1) * 1000 + (point.x + 1) * 4 + direction
}

fn modulo(a: i32, b: i32) -> i32 {
    let mut rem = a % b;
    if rem < 0 {
        rem += b;
    }
    rem
}

sample!(d22, 6032, 5031, "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
", 4);
