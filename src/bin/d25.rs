use aoc::{main, sample};

main!(d25, "Full of Hot Air");

fn d25(input: &str) -> (String, i64) {
    let total = input.lines()
        .map(snafu2dec)
        .sum();
    (dec2snafu(total), 0)
}

fn snafu2dec(snafu: &str) -> i64 {
    snafu.chars()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!()
        })
        .reduce(|acc, n| acc * 5 + n)
        .unwrap()
}

fn dec2snafu(mut dec: i64) -> String {
    let mut string = String::new();
    while dec > 0 {
        string.push(match dec % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!()
        });
        dec = (dec + 2) / 5;
    }
    string.chars().rev().collect()
}

sample!(d25, "2=-1=0", 0, "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
");
