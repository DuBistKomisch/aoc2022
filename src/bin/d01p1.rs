fn main() {
    let mut current: u32 = 0;
    let mut most: u32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        match line.parse::<u32>() {
            Ok(x) => current += x,
            Err(_) => {
                if current > most {
                    most = current;
                }
                current = 0;
            }
        }
    }
    println!("{}", most);
}
