use aoc::{main, sample};

main!(d08, "Treetop Tree House");

fn d08(input: &str) -> (usize, usize) {
    let heights: Vec<Vec<usize>> = input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect())
        .collect();
    (part1(&heights), part2(&heights))
}

fn part1(heights: &Vec<Vec<usize>>) -> usize {
    let mut visible: Vec<Vec<bool>> = vec![vec![false; heights[0].len()]; heights.len()];
    for y in 0..heights.len() {
        let mut blocked = -1;
        for x in 0..heights[y].len() {
            let height = heights[y][x] as i32;
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
        blocked = -1;
        for x in (0..heights[y].len()).rev() {
            let height = heights[y][x] as i32;
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
    }
    for x in 0..heights[0].len() {
        let mut blocked = -1;
        for y in 0..heights.len() {
            let height = heights[y][x] as i32;
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
        blocked = -1;
        for y in (0..heights.len()).rev() {
            let height = heights[y][x] as i32;
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
    }
    visible.iter().map(|row| row.iter().filter(|&v| *v).count()).sum()
}

fn part2(heights: &Vec<Vec<usize>>) -> usize {
    let mut scenic: Vec<Vec<usize>> = vec![vec![1; heights[0].len()]; heights.len()];
    for y in 0..heights.len() {
        let mut last = vec![0; 10];
        for x in 0..heights[y].len() {
            let height = heights[y][x];
            let distance = x - last[height];
            scenic[y][x] *= distance;
            for i in 0..=height {
                last[i] = x;
            }
        }
        let mut last = vec![heights[y].len() - 1; 10];
        for x in (0..heights[y].len()).rev() {
            let height = heights[y][x];
            let distance = last[height] - x;
            scenic[y][x] *= distance;
            for i in 0..=height {
                last[i] = x;
            }
        }
    }
    for x in 0..heights[0].len() {
        let mut last = vec![0; 10];
        for y in 0..heights.len() {
            let height = heights[y][x];
            let distance = y - last[height];
            scenic[y][x] *= distance;
            for i in 0..=height {
                last[i] = y;
            }
        }
        let mut last = vec![heights.len() - 1; 10];
        for y in (0..heights.len()).rev() {
            let height = heights[y][x];
            let distance = last[height] - y;
            scenic[y][x] *= distance;
            for i in 0..=height {
                last[i] = y;
            }
        }
    }
    *scenic.iter().filter_map(|row| row.iter().max()).max().unwrap()
}

sample!(d08, 21, 8, "\
30373
25512
65332
33549
35390
");
