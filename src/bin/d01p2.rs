fn main() {
    let mut current: u32 = 0;
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        match line.parse::<u32>() {
            Ok(x) => current += x,
            Err(_) => {
                if current > first {
                    third = second;
                    second = first;
                    first = current;
                } else if current > second {
                    third = second;
                    second = current;
                } else if current > third {
                    third = current;
                }
                current = 0;
            }
        }
    }
    println!("{}", first + second + third);
}
