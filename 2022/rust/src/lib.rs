use std::time::{Duration, Instant};

// Days
pub mod helper;
pub mod days;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

pub fn run_day(day: u32, input: &str) ->
    ( Option<Duration>,
      Option<(Duration, String)>,
      Option<(Duration, String)> )
{
    match day {
         1 => run_day!(day01, input),
         2 => run_day!(day02, input),
         3 => run_day!(day03, input),
         4 => run_day!(day04, input),
         5 => run_day!(day05, input),
         6 => run_day!(day06, input),
         7 => run_day!(day07, input),
         8 => run_day!(day08, input),
         9 => run_day!(day09, input),
        10 => run_day!(day10, input),
        11 => run_day!(day11, input),
        12 => run_day!(day12, input),
        13 => run_day!(day13, input),
        14 => run_day!(day14, input),
        15 => run_day!(day15, input),
        _ => {
            return (None, None, None);
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

#[macro_export]
macro_rules! run_parse {
    ($func:expr, $input:expr) => {
        {
            let start = Instant::now();
            let result = $func($input);
            let dur = start.elapsed();
            (dur, result)
        }
    };
}

#[macro_export]
macro_rules! run_day {
    ($mod:ident, $input:expr) => {
        {
            let (parse_d,parsed) = run_parse!(days::$mod::parse, $input);
            let part1 = run_part!(days::$mod::part1, parsed.clone());
            let part2 = run_part!(days::$mod::part2, parsed.clone());
            return (Some(parse_d), Some(part1), Some(part2));
        }
    };
}
