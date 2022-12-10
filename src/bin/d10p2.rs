fn main() {
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        match &line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => {
                draw(x, cycle);
                cycle += 1;
            },
            ["addx", v] => {
                draw(x, cycle);
                cycle += 1;
                draw(x, cycle);
                x += v.parse::<i32>().unwrap();
                cycle += 1;
            },
            _ => unreachable!()
        }
    }
}

fn draw(x: i32, cycle: i32) {
    if ((cycle - 1) % 40 - x).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if cycle % 40 == 0 {
        println!()
    }
}
