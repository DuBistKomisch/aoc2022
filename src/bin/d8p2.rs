fn main() {
    let heights: Vec<Vec<usize>> = std::io::stdin().lines()
        .map(|line| line.unwrap().chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect())
        .collect();
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
    let answer = scenic.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    println!("{}", answer);
}
