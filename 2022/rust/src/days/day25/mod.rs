#![allow(unused_imports)]
use std::collections::{HashSet, HashMap, VecDeque};
use crate::helper::{nums_in_str, first_num_in_str};
use itertools::Itertools;

type Input = Vec<isize>;

pub fn parse(input: &str) -> Input {
    return input.lines().map(|l| {
        let mut num : isize = 0;
        l.chars().rev().enumerate().for_each(|(i,c)| {
            num += (match c {
                '=' => -2,
                '-' => -1,
                '0' =>  0,
                '1' =>  1,
                '2' =>  2,
                _ => panic!("UNKNOWN")
            } * 5isize.pow(i as u32))
        });
        num
    }).collect::<Input>();
}

fn to_snafu(num: isize) -> String {
    if num == 0 {
        return "".to_string();
    }

    let rem = num % 5;
    for (c,r) in [('=',-2), ('-',-1), ('0', 0), ('1', 1), ('2', 2)] {
        if (r + 5) % 5 == rem {
            let mut c_str = to_snafu((num-r)/5);
            c_str.push(c);
            return c_str;
        }
    }

    panic!("You shouldn't be here...");
}

pub fn part1(input: Input) -> String {
    let sum : isize = input.iter().sum();
    return to_snafu(sum);
}

pub fn part2(_: Input) -> String {
    return "Merry Christmas!".to_string();
}
