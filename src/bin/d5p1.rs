fn main() {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut lines = std::io::stdin().lines();
    'crates: while let Some(line) = lines.next() {
        for (i, c) in line.unwrap().chars().skip(1).step_by(4).enumerate() {
            if c.is_numeric() {
                break 'crates;
            }
            if c.is_whitespace() {
                continue;
            }
            stacks[i].insert(0, c);
        }
    }
    lines.next();
    for line in lines {
        let parts: Vec<usize> = line.unwrap().split(' ').filter_map(|n| n.parse().ok()).collect();
        if let [n, from, to] = &parts[..] {
            let at = stacks[*from - 1].len() - n;
            let mut moved = stacks[*from - 1].split_off(at);
            moved.reverse();
            stacks[*to - 1].append(&mut moved);
        }
    }
    let answer: String = stacks.iter().filter_map(|stack| stack.last()).collect();
    println!("{}", answer);
}
