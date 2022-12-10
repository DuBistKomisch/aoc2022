fn main() {
    let mut lines = std::io::stdin().lines();
    if let Some(line) = lines.next() {
        let chars: Vec<char> = line.unwrap().chars().collect();
        let offset = 4 + chars.windows(4).position(|quad| {
            let mut vec = quad.to_vec();
            vec.sort();
            vec.dedup();
            vec.len() == 4
        }).unwrap();
        println!("{}", offset);
    }
}
