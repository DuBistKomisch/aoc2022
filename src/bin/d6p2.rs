fn main() {
    let mut lines = std::io::stdin().lines();
    if let Some(line) = lines.next() {
        let chars: Vec<char> = line.unwrap().chars().collect();
        let offset = 14 + chars.windows(14).position(|quad| {
            let mut vec = quad.to_vec();
            vec.sort();
            vec.dedup();
            vec.len() == 14
        }).unwrap();
        println!("{}", offset);
    }
}
