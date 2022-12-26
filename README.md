# aoc2022

My solutions to [Advent of Code 2022](https://adventofcode.com/2022) in Rust.

## Run

Install and use nightly Rust using rustup.

Run tests with sample data using e.g. `cargo test --bin d01`.

Run with input using e.g. `cargo run --bin d01 < input/d01` (BYO input files).

## Goals

- Practice Rust!
  - I'll come back and refactor as I learn more tricks.
- Use only the standard library, no crates.
  - Next time I might allow `itertools` for convenience though.
  - Nightly is used mostly for the new const generics chunk methods which make input parsing a lot more readable.
- Solve the problems generically, don't abuse my particular input.
  - The problems often (deliberately) don't exhaustively cover every edge case or fully specify the input format, so I just make my best guess.
  - For example, part 2 of day 22 seems to be intended to be solved by hardcoding for your input (at least if aiming for speed), but I solved it generically because it's much more challenging and interesting.
  - Another example is day 7, where the input always follows a certain pattern, allowing you to skip a lot of work. I assumed any legal input was possible as long as it eventually fully explored the tree.

Non-goals:

- Place on the leaderboard.
  - I don't have the free time for that, and I'd rather just have fun.
- Find the optimal solution.
  - I'll try of course, but I'm happy with something which works.
  - For example, day 19 takes 14 minutes to run, and optimising it any more isn't interesting.
  - I won't go back to change my strategy afterwards, only refactor the existing solution.
