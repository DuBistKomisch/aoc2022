fn main() {
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut answer: i32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        match &line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => {
                answer += signal(x, cycle);
                cycle += 1;
            },
            ["addx", v] => {
                answer += signal(x, cycle);
                cycle += 1;
                answer += signal(x, cycle);
                x += v.parse::<i32>().unwrap();
                cycle += 1;
            },
            _ => unreachable!()
        }
    }
    println!("{}", answer);
}

fn signal(x: i32, cycle: i32) -> i32 {
    if (cycle - 20) % 40 == 0 {
        cycle * x
    } else {
        0
    }
}
