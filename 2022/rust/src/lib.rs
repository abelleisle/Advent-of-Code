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
