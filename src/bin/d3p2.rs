#![feature(iter_array_chunks)]

fn main() -> std::io::Result<()> {
    let mut sum: u32 = 0;
    for [line1, line2, line3] in std::io::stdin().lines().array_chunks() {
        let seen: std::collections::HashSet<char> = line1?.chars().collect();
        let twice: std::collections::HashSet<char> = line2?.chars().filter(|c| seen.contains(c)).collect();
        let badge = line3?.chars().find(|c| twice.contains(c)).unwrap();
        let priority = match badge {
            'a'..='z' => badge as u32 - 'a' as u32 + 1,
            'A'..='Z' => badge as u32 - 'A' as u32 + 27,
            _ => 0
        };
        sum += priority;
    }
    println!("{}", sum);
    Ok(())
}
