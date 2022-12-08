# Advent of Code Rust Starter

[![Run on Repl.it](https://repl.it/badge/github/ljgago/advent-of-code-rust-starter)](https://repl.it/github/ljgago/advent-of-code-rust-starter)

A tamplate for [Advent of Code](https://adventofcode.com) write in Rust.

- [Day 1](src/day01)
- [Day 2](src/day02)
- [Day 3](src/day03)
- [Day 4](src/day04)
- [Day 5](src/day05)
- [Day 6](src/day06)
- [Day 7](src/day07)
- [Day 8](src/day08)
- [Day 9](src/day09)
- [Day 10](src/day10)
- [Day 11](src/day11)
- [Day 12](src/day12)
- [Day 13](src/day13)
- [Day 14](src/day14)
- [Day 15](src/day15)
- [Day 16](src/day16)
- [Day 17](src/day17)
- [Day 18](src/day18)
- [Day 19](src/day19)
- [Day 20](src/day20)
- [Day 21](src/day21)
- [Day 22](src/day22)
- [Day 23](src/day23)
- [Day 24](src/day24)
- [Day 25](src/day25)

## Usage

The project is configured for `run`, `test` and `build` each day independently.

For example:

    # only run the day01
    $ cargo run --bin day01

    # only test the day02
    $ cargo test --bin day02

    # only build the day03
    $ cargo build --bin day03

Folder structure:

     src
     └── day01
         ├── bin
         │   ├── day01.rs
         │   ├── part1.rs
         │   └── part2.rs
         ├── input.txt
         └── README.md

## nix integration

You can use `nix-shell` to install the rust and tool dependencies used to run
the project.

## Replit integration

You can edit the `.replit` file and change the `run` command for different
puzzles each day.

For example:

    # test the day01
    run = "cargo test --bin day01"

    # test the day02 and run the day02 if the test pass
    run = "cargo test --bin day02 && cargo run --bin day02"

Happy coding!

[MIT License](LICENSE)
