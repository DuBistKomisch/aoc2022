#![feature(iter_next_chunk)]

use aoc::{main, sample};

main!(d15, "Beacon Exclusion Zone");

#[derive(Clone)]
struct IntervalSet {
    intervals: Vec<(i64, i64)>
}

impl IntervalSet {
    fn new() -> Self {
        Self { intervals: vec![] }
    }

    fn union(&mut self, interval: (i64, i64)) {
        self.intervals.push(interval);
        self.intervals.sort();
        let mut i = self.intervals.len() - 1;
        while i > 0 {
            // if overlaps (or touches exactly) previous interval, combine
            if self.intervals[i-1].1 >= self.intervals[i].0 - 1 {
                if self.intervals[i].1 > self.intervals[i-1].1 {
                    self.intervals[i-1].1 = self.intervals[i].1;
                }
                self.intervals.remove(i);
                // try again to combine with next interval, unless already last
                if i < self.intervals.len() {
                    continue;
                }
            }
            i -= 1;
        }
    }

    fn subtract(&mut self, interval: (i64, i64)) {
        let mut i = self.intervals.len() - 1;
        loop {
            if interval.0 <= self.intervals[i].0 && interval.1 >= self.intervals[i].1 {
                // full overlap, remove
                self.intervals.remove(i);
            } else if interval.0 > self.intervals[i].0 && interval.1 < self.intervals[i].1 {
                // in the middle, split
                self.intervals.insert(i + 1, (interval.1 + 1, self.intervals[i].1));
                self.intervals[i].1 = interval.0 - 1;
            } else if interval.0 > self.intervals[i].0 && interval.0 <= self.intervals[i].1 {
                // overlaps right, trim
                self.intervals[i].1 = interval.0 - 1;
            } else if interval.1 < self.intervals[i].1 && interval.1 >= self.intervals[i].0 {
                // overlaps left, trim
                self.intervals[i].0 = interval.1 + 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    fn len(&self) -> i64 {
        self.intervals.iter().map(|(from, to)| to - from + 1).sum()
    }
}

fn d15(input: &str) -> (i64, i64) {
    let data: Vec<[[i64; 2]; 2]> = input.lines().map(|line| line
            .strip_prefix("Sensor at ").unwrap()
            .split(": closest beacon is at ")
            .map(|coords| coords.split(", ")
                 .map(|coord| coord.split('=')
                      .nth(1).unwrap()
                      .parse().unwrap())
                 .next_chunk().unwrap())
            .next_chunk().unwrap())
        .collect();
    let max_sx = data.iter()
        .map(|[[sx, _], _]| sx)
        .max().unwrap();
    let threshold = if *max_sx < 100 { 10 } else { 2_000_000 };
    (part1(data.clone(), threshold), part2(data, threshold))
}

fn part1(data: Vec<[[i64; 2]; 2]>, threshold: i64) -> i64 {
    let mut excluded = IntervalSet::new();
    for [[sx, sy], [bx, by]] in data {
        let distance = sx.abs_diff(bx) + sy.abs_diff(by);
        if sy.abs_diff(threshold) < distance {
            let range = (distance - sy.abs_diff(threshold)) as i64;
            excluded.union((sx - range, sx + range));
        }
        if by == threshold {
            excluded.subtract((bx, bx));
        }
    }
    excluded.len()
}

fn part2(data: Vec<[[i64; 2]; 2]>, threshold: i64) -> i64 {
    let mut rows = vec![IntervalSet::new(); (threshold * 2 + 1) as usize];
    for [[sx, sy], [bx, by]] in data {
        let distance = sx.abs_diff(bx) + sy.abs_diff(by);
        for y in (sy - distance as i64).max(0)..=(sy + distance as i64).min(threshold * 2) {
            let range = (distance - sy.abs_diff(y)) as i64;
            let left = (sx - range).max(0);
            let right = (sx + range).min(threshold * 2);
            rows[y as usize].union((left, right));
        }
    }
    let y = rows.iter().position(|row| row.len() == threshold * 2).unwrap();
    let x = rows[y].intervals[0].1 + 1;
    x * 4_000_000 + y as i64
}

sample!(d15, 26, 56000011, "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
");
