fn main() {
    let mut count: u32 = 0;
    for line in std::io::stdin().lines() {
        let parts: Vec<u32> = line.unwrap().split(&['-', ',']).map(|n| n.parse().unwrap()).collect();
        if let [a1, a2, b1, b2] = &parts[..] {
            if a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
