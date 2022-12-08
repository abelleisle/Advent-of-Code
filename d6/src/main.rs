#![allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;

use std::collections::HashSet;

fn first_unique <T: std::hash::Hash + std::cmp::Eq> (v: &Vec<T>, n: usize) -> Option<usize> {
    for i in 0..v.len()-n {
        let s = &v[i..i+n];
        let h : HashSet<&T> = s.iter().collect();
        if h.len() == n {
            return Some(i + n);
        }
    }
    None
}

fn main() {
    let input : Vec<char> = include_str!("../input.txt").chars().collect();

    println!("Part 1: {}", first_unique(&input, 4).unwrap());
    println!("Part 2: {}", first_unique(&input, 14).unwrap());
}
