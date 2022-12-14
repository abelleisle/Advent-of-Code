use std::time::{Duration, Instant};

// Days
pub mod helper;
pub mod days;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

pub fn run_day(day: u32, input: &str) ->
    ( Option<(Duration, String)>,
      Option<(Duration, String)> )
{
    match day {
        1 => {
            let parsed = days::day01::parse(input);
            let part1 = run_part!(days::day01::part1, parsed.clone());
            let part2 = run_part!(days::day01::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        2 => {
            let parsed = days::day02::parse(input);
            let part1 = run_part!(days::day02::part1, parsed.clone());
            let part2 = run_part!(days::day02::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        3 => {
            let parsed = days::day03::parse(input);
            let part1 = run_part!(days::day03::part1, parsed.clone());
            let part2 = run_part!(days::day03::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        4 => {
            let parsed = days::day04::parse(input);
            let part1 = run_part!(days::day04::part1, parsed.clone());
            let part2 = run_part!(days::day04::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        5 => {
            let parsed = days::day05::parse(input);
            let part1 = run_part!(days::day05::part1, parsed.clone());
            let part2 = run_part!(days::day05::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        6 => {
            let parsed = days::day06::parse(input);
            let part1 = run_part!(days::day06::part1, parsed.clone());
            let part2 = run_part!(days::day06::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        7 => {
            let parsed = days::day07::parse(input);
            let part1 = run_part!(days::day07::part1, parsed.clone());
            let part2 = run_part!(days::day07::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        8 => {
            let parsed = days::day08::parse(input);
            let part1 = run_part!(days::day08::part1, parsed.clone());
            let part2 = run_part!(days::day08::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        9 => {
            let parsed = days::day09::parse(input);
            let part1 = run_part!(days::day09::part1, parsed.clone());
            let part2 = run_part!(days::day09::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        10 => {
            let parsed = days::day10::parse(input);
            let part1 = run_part!(days::day10::part1, parsed.clone());
            let part2 = run_part!(days::day10::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        11 => {
            let parsed = days::day11::parse(input);
            let part1 = run_part!(days::day11::part1, parsed.clone());
            let part2 = run_part!(days::day11::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        12 => {
            let parsed = days::day12::parse(input);
            let part1 = run_part!(days::day12::part1, parsed.clone());
            let part2 = run_part!(days::day12::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        13 => {
            let parsed = days::day13::parse(input);
            let part1 = run_part!(days::day13::part1, parsed.clone());
            let part2 = run_part!(days::day13::part2, parsed.clone());
            return (Some(part1), Some(part2));
        },
        _ => {
            return (None, None);
        }
    }
}

#[macro_export]
macro_rules! run_part {
    ($func:expr, $input:expr) => {
        {
            let start = Instant::now();
            let result = $func($input).to_string().trim_end().to_string();
            let dur = start.elapsed();
            (dur, result)
        }
    };
}
