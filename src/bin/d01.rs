use aoc::{main, sample};

main!(d01, "Calorie Counting");

fn d01(input: &str) -> (u32, u32) {
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;
    for elf in input.split("\n\n") {
        let current = elf.lines()
            .filter_map(|line| line.parse::<u32>().ok())
            .sum();
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
    }
    (first, first + second + third)
}

sample!(d01, 24000, 45000, "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
");
