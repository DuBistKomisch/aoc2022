fn main() -> std::io::Result<()> {
    let mut sum: u32 = 0;
    for line in std::io::stdin().lines() {
        let line = line?;
        let compartment = line.chars().count() / 2;
        let seen: std::collections::HashSet<char> = line.chars().take(compartment).collect();
        let shared = line.chars().skip(compartment).find(|c| seen.contains(c)).unwrap();
        let priority = match shared {
            'a'..='z' => shared as u32 - 'a' as u32 + 1,
            'A'..='Z' => shared as u32 - 'A' as u32 + 27,
            _ => 0
        };
        sum += priority;
    }
    println!("{}", sum);
    Ok(())
}
