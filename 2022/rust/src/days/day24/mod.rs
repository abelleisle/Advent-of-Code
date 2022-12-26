#![allow(unused_imports)]
use std::collections::{HashSet, HashMap, VecDeque};
use crate::helper::{nums_in_str, first_num_in_str};
use itertools::Itertools;

type Pos = (isize, isize);

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Valley {
    width:   isize,
    height:  isize,
    entrance: Pos,
    exit:    Pos,
    blizzards: Vec<(Pos, Pos)>,
    repeat: usize
}

impl Valley {
    fn new(s: &str) -> Self {
        let mut v = Valley {
            width: 0,
            height: 0,
            entrance: (1,0),
            exit: (1,0),
            blizzards: vec![],
            repeat: usize::MAX
        };

        // s.lines().enumerate();
        s.lines().enumerate().for_each(|(y,l)| {
            let y = y as isize - 1;
            v.height = y;
            l.chars().enumerate().for_each(|(x,c)| {
                let x = x as isize - 1;
                match c {
                    '>' => v.blizzards.push(((x,y), ( 1, 0))),
                    '<' => v.blizzards.push(((x,y), (-1, 0))),
                    'v' => v.blizzards.push(((x,y), ( 0, 1))),
                    '^' => v.blizzards.push(((x,y), ( 0,-1))),
                    '.' => {
                        if y == -1         {v.entrance = (x, y)}
                        if y == v.height   {v.exit =     (x, y)}
                    },
                    '#' => {},
                    _ => panic!("FRIG")
                }
                v.width = x;
            });
        });

        let mut rotate = v.clone();
        rotate.step();
        v.repeat = 1;
        while rotate.blizzards != v.blizzards {
            rotate.step();
            v.repeat += 1;
        }
        println!("Width:    {}", v.width);
        println!("Height:   {}", v.height);
        println!("Entrance: {},{}", v.entrance.0, v.entrance.1);
        println!("Exit:     {},{}", v.exit.0, v.exit.1);
        println!("Rotates:  {}", v.repeat);

        return v;
    }

    fn step(self: &mut Self) {
        for (p,d) in self.blizzards.iter_mut() {
            p.0 = (p.0 + d.0).rem_euclid(self.width);
            p.1 = (p.1 + d.1).rem_euclid(self.height);
        }
    }

    fn free(self: &Self, pos: Pos) -> bool {
        return !self.blizzards.iter().any(|&b| b.0 == pos);
    }
}

type Input = Valley;

pub fn parse(input: &str) -> Input {
//     let input
//     = r"#.######
// #>>.<^<#
// #.<..<<#
// #>v.><>#
// #<^v^^>#
// ######.#";

    return Valley::new(input);
}

fn solve(initial_valley: Input, start: Pos, goal: Pos, time: usize) -> (usize, Input) {
    let mut s : HashSet<(Pos, usize)> = HashSet::new();
    let mut q : VecDeque<(Pos, usize, Input)> = VecDeque::from(
        [(start, time, initial_valley.clone())]
    );

    while let Some(t) = q.pop_front() {
        let (p,d,mut v) = t;
        if d >= 1000 {continue;};

        if p == goal {
            return (d,v);
        }

        if !v.free(p) {
            continue;
        }

        if s.contains(&(p, d%v.repeat)) {
            continue;
        }
        s.insert((p, d%v.repeat));

        v.step();

        // Wait
        if v.free(p) {
            q.push_back((p, d+1, v.clone()));
        }

        if p == v.entrance {
            if v.free((p.0, p.1+1)) {
                q.push_back(((p.0, p.1+1), d+1, v.clone()));
            }
        } else if p == v.exit {
            if v.free((p.0, p.1-1)) {
                q.push_back(((p.0, p.1-1), d+1, v.clone()));
            }
        } else {
            // Go down..
            if (p.1 < v.height -1 && v.free((p.0, p.1+1))) ||
               (p.0 == v.exit.0 && p.1 == v.exit.1 - 1) {
                q.push_back(((p.0, p.1+1), d+1, v.clone()));
            }
            // Go up..
            if (p.1 > 0 && v.free((p.0, p.1-1))) ||
               (p.0 == v.entrance.0 && p.1 == v.entrance.1 + 1) {
                q.push_back(((p.0, p.1-1), d+1, v.clone()));
            }
            // Go right
            if p.0 < v.width - 1 && v.free((p.0+1, p.1)) {
                q.push_back(((p.0+1, p.1), d+1, v.clone()));
            }
            // Go left
            if p.0 > 0 && v.free((p.0-1, p.1)) {
                q.push_back(((p.0-1, p.1), d+1, v.clone()));
            }
        }
    }

    return (0,initial_valley.clone());
}

pub fn part1(input: Input) -> usize {
    let s = input.entrance;
    let e = input.exit;
    return solve(input, s, e, 0).0;
}

pub fn part2(input: Input) -> usize {
    let s = input.entrance;
    let e = input.exit;
    let r0 = solve(input, s, e, 0);
    let r1 = solve(r0.1,  e, s, r0.0);
    let r2 = solve(r1.1,  s, e, r1.0);
    return r2.0;
}
