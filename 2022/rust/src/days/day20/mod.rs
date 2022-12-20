#![allow(unused_imports)]
use std::collections::{HashSet, HashMap, VecDeque};
use crate::helper::{nums_in_str, first_num_in_str};
use itertools::Itertools;

type Input = Vec<isize>;

pub fn parse(input: &str) -> Input {
    return input.lines().map(|l| l.parse::<isize>().unwrap()).collect::<Input>().clone();
}

fn rearrange(nums: &Vec<isize>, i: usize, k: isize) -> isize {
    let input = nums.iter().map(|x| x * k).collect::<Vec<_>>(); // Numbers
    let mut to_arrange = (0..input.len()).collect::<Vec<_>>();  // Indexes
    for _ in 0..i {
        for (i, &n) in input.iter().enumerate() {
            let pos = to_arrange.iter().position(|&r| r == i).unwrap();
            to_arrange.remove(pos);
            let tp : usize = (pos as isize + n).rem_euclid(to_arrange.len() as isize) as usize;
            to_arrange.insert(tp, i);
        }
    }
    let opos = input.iter().position(|&i| i == 0).unwrap();
    let zpos = to_arrange.iter().position(|&i| i == opos).unwrap();
    return input[to_arrange[(zpos+1000)%to_arrange.len()]] +
           input[to_arrange[(zpos+2000)%to_arrange.len()]] +
           input[to_arrange[(zpos+3000)%to_arrange.len()]];
}

pub fn part1(input: Input) -> isize {
    return rearrange(&input, 1, 1);
}

pub fn part2(input: Input) -> isize {
    return rearrange(&input, 10, 811589153);
}
