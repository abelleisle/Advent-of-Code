#![allow(unused_imports)]
use std::collections::{HashSet, HashMap, VecDeque};
use crate::helper::{nums_in_str, first_num_in_str};
use itertools::Itertools;

type Input<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Input {
    return input.lines().collect::<Input>().clone();
}

pub fn part1(input: Input) -> usize {
    return input.len();
}

pub fn part2(input: Input) -> usize {
    return input.len() + 1;
}
