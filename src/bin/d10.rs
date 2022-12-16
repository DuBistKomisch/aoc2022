use aoc::{main, sample};

main!(d10, "Cathode-Ray Tube");

fn d10(input: &str) -> (i32, String) {
    let mut part1 = 0;
    process(input, |x, cycle| {
        if (cycle - 20) % 40 == 0 {
            part1 += cycle * x;
        }
    });
    let mut part2 = String::new();
    process(input, |x, cycle| {
        if x.abs_diff((cycle - 1) % 40) <= 1 {
            part2 += "#";
        } else {
            part2 += ".";
        }
        if cycle % 40 == 0 {
            part2 += "\n";
        }
    });
    (part1, part2)
}

fn process<F>(input: &str, mut sample: F)
where
    F: FnMut(i32, i32)
{
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    for line in input.lines() {
        match &line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => {
                sample(x, cycle);
                cycle += 1;
            },
            ["addx", v] => {
                sample(x, cycle);
                cycle += 1;
                sample(x, cycle);
                x += v.parse::<i32>().unwrap();
                cycle += 1;
            },
            _ => unreachable!()
        }
    }
}

sample!(d10, 13140, "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
", "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
");
