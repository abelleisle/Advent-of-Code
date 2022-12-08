#![allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

use std::env;
use std::fs;
use std::io;
use std::time::{Duration, Instant};

use core::any::Any;

use advent_of_code;

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}

fn solve_day(day_num: u32) {
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("src/days").join(format!("day{:02}/input.txt", day_num));
    // println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename.clone());

    println!("{}-- Day {} --{}",
        advent_of_code::ANSI_BOLD,
        day_num,
        advent_of_code::ANSI_RESET);

    if input.is_err() {
        println!("  {}Warning:{} Day {} input file missing..",
            advent_of_code::ANSI_BOLD,
            advent_of_code::ANSI_RESET,
            day_num);
        println!("  {}", filename.display());
        return;
    }

    let to_run = advent_of_code::run_day(day_num, &input.unwrap());

    if to_run.0.is_none() && to_run.1.is_none() {
        println!("  Not implemented");
        return;
    }

    // Run part 1
    if let Some(part1) = to_run.0 {
        println!("  {}Part 1{}", advent_of_code::ANSI_BOLD, advent_of_code::ANSI_RESET);
        println!("    Result: {}  {}{}{}\n",
            part1.1,
            advent_of_code::ANSI_ITALIC,
            fmt_dur(part1.0),
            advent_of_code::ANSI_RESET);
    }

    // Run part 1
    if let Some(part2) = to_run.1 {
        println!("  {}Part 2{}", advent_of_code::ANSI_BOLD, advent_of_code::ANSI_RESET);
        println!("    Result: {}  {}{}{}\n",
            part2.1,
            advent_of_code::ANSI_ITALIC,
            fmt_dur(part2.0),
            advent_of_code::ANSI_RESET);
    }
}

fn main() {
    // Get day string
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let mut day = args[1].clone();
        // Parse day as number
        day = day.trim().to_string();
        let day_num: u32 = match day.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid day number: {}", day);
                return;
            }
        };
        solve_day(day_num);
    } else {
        for d in 1..=25 {
            solve_day(d);
        }
    }


}
