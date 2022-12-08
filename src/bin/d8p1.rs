fn main() {
    let heights: Vec<Vec<i32>> = std::io::stdin().lines()
        .map(|line| line.unwrap().chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect())
        .collect();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; heights[0].len()]; heights.len()];
    for y in 0..heights.len() {
        let mut blocked = -1;
        for x in 0..heights[y].len() {
            let height = heights[y][x];
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
        blocked = -1;
        for x in (0..heights[y].len()).rev() {
            let height = heights[y][x];
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
    }
    for x in 0..heights[0].len() {
        let mut blocked = -1;
        for y in 0..heights.len() {
            let height = heights[y][x];
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
        blocked = -1;
        for y in (0..heights.len()).rev() {
            let height = heights[y][x];
            if height > blocked {
                visible[y][x] = true;
                blocked = height;
            }
        }
    }
    let answer: usize = visible.iter().map(|row| row.iter().filter(|&v| *v).count()).sum();
    println!("{}", answer);
}
