#![feature(iter_next_chunk)]

use aoc::{main, sample};

main!(d02, "Rock Paper Scissors");

#[derive(Clone, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock
        }
    }
}

fn d02(input: &str) -> (u32, u32) {
    (
        solve(input, |ours, _theirs| match ours {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => unreachable!()
        }),
        solve(input, |ours, theirs| match ours {
            "X" => theirs.beats(),
            "Y" => theirs.clone(),
            "Z" => theirs.beaten_by(),
            _ => unreachable!()
        })
    )
}

fn solve<F>(input: &str, pick: F) -> u32
where
    F: Fn(&str, &RPS) -> RPS
{
    input.lines().map(|line| {
        let [theirs, ours] = line.split_whitespace().next_chunk().unwrap();
        let theirs = match theirs {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => unreachable!()
        };
        let ours = pick(ours, &theirs);
        let mut score = ours.value();
        if theirs == ours {
            score += 3;
        }
        if theirs.beaten_by() == ours {
            score += 6;
        }
        score
    }).sum()
}

sample!(d02, 15, 12, "\
A Y
B X
C Z
");
