#[macro_export]
macro_rules! main {
    ($solve:ident, $name:expr $(, $opt:expr)*) => {
        use std::io::Read;
        fn main() {
            println!("{}", $name);
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let (part1, part2) = $solve(&input, $($opt),*);
            println!("Part 1:");
            println!("{}", part1);
            println!("Part 2:");
            println!("{}", part2);
        }
    }
}

#[macro_export]
macro_rules! sample {
    ($solve:ident, $part1:expr, $part2:expr, $input:expr $(, $opt: expr)*) => {
        #[test]
        fn sample() {
            const INPUT: &str = $input;
            let (part1, part2) = $solve(INPUT, $($opt),*);
            // these are done separately to allow &str == String
            assert_eq!(part1, $part1);
            assert_eq!(part2, $part2);
        }
    }
}
