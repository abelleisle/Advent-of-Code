#![allow(unused_imports)]
use std::collections::{HashSet, HashMap, VecDeque};
use crate::helper::{nums_in_str, first_num_in_str};
use itertools::Itertools;

type Pos = (isize, isize);
type Input = HashSet<Pos>;

pub fn parse(input: &str) -> Input {
    let mut output : Input = HashSet::new();
    for (y, yc) in input.lines().enumerate() {
        for (x, xc) in yc.chars().enumerate() {
            if xc == '#' {
                output.insert((x as isize, y as isize));
            }
        }
    }
    return output;
}

fn bounds(elves: &Input) -> (isize, isize, isize, isize) {
    let (xs,xe) = elves.iter().map(|k| k.0).minmax().into_option().unwrap();
    let (ys,ye) = elves.iter().map(|k| k.1).minmax().into_option().unwrap();
    return (xs,xe,ys,ye);
}

pub fn at(elves: &Input, pos: Pos) -> bool {
    return elves.get(&pos).is_some();
}

pub fn solve(elves: &mut Input, round: usize) -> usize {
    let mut et : HashMap<Pos, Vec<Pos>> = HashMap::new();
    let to_try = vec![
        (( 0,-1),(-1,-1),( 1,-1)),
        (( 0, 1),(-1, 1),( 1, 1)),
        ((-1, 0),(-1, 1),(-1,-1)),
        (( 1, 0),( 1, 1),( 1,-1)),
    ];

    for e in elves.iter() {
        let (x,y) = e;

        if !to_try.iter().any(|e| {
            let t = vec![(x + e.0.0, y + e.0.1),
                         (x + e.1.0, y + e.1.1),
                         (x + e.2.0, y + e.2.1)];
            return t.iter().any(|i| at(elves, *i));
        }) {
            continue;
        }

        'inner: for d in 0..4 {
            let r = (round + d)%4;
            let g = to_try[r];
            let t = ((x + g.0.0, y + g.0.1),
                     (x + g.1.0, y + g.1.1),
                     (x + g.2.0, y + g.2.1));
            if !at(elves, t.0) && !at(elves, t.1) && !at(elves, t.2) {
                et.entry(t.0).or_default().push(*e);
                break 'inner;
            }
        }
    }

    let mut moved = 0;
    for (d,e) in et {
        if e.len() == 1 {
            elves.remove(&e[0]);
            elves.insert(d);
            moved += 1;
        }
    }
    return moved;
}

pub fn part1(mut input: Input) -> isize {
    for r in 0..10 {
        solve(&mut input, r);
    }

    let (xs, xe, ys, ye) = bounds(&input);
    return ((1+xe-xs) * (1+ye-ys)) - input.len() as isize;
}

pub fn part2(mut input: Input) -> usize {
    let mut r = 0;
    loop {
        if solve(&mut input, r) == 0 {
            return r + 1;
        }
        r += 1;
    }
}
