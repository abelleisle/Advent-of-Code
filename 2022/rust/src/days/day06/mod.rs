type Input = Vec<char>;

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

pub fn parse(input: &str) -> Input {
    return input.chars().collect::<Input>().clone();
}

pub fn part1(input: Input) -> usize {
    return first_unique(&input, 4).unwrap();
}

pub fn part2(input: Input) -> usize {
    return first_unique(&input, 14).unwrap();
}
