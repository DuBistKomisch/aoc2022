#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

fn rps_value(rps: &RPS) -> u32 {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3
    }
}

fn rps_beaten_by(rps: &RPS) -> RPS {
    match rps {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock
    }
}

fn main() {
    let mut score: u32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let theirs = match &line[0..1] {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("invalid theirs")
        };
        let ours = match &line[2..3] {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("invalid ours")
        };
        score += rps_value(&ours);
        if theirs == ours {
            score += 3;
        }
        if rps_beaten_by(&theirs) == ours {
            score += 6;
        }
    }
    println!("{}", score);
}
