#![allow(unused_imports)]
use std::collections::{HashSet, HashMap, VecDeque};
use crate::helper::{nums_in_str, first_num_in_str};
use itertools::Itertools;

#[derive(Clone)]
pub enum Monkey {
    Op(String, String, char),
    Num(i128)
}

type Input = HashMap<String, Monkey>;

pub fn parse(input: &str) -> Input {
    let mut map : Input = HashMap::new();
    input.lines().for_each(|l| {
        let m = l.split_once(": ").unwrap();
        let k = m.0.to_string();
        if let Ok(x) = m.1.parse::<i128>() {
            map.insert(k, Monkey::Num(x));
        } else {
            let w = m.1.split_whitespace().collect::<Vec<_>>();
            map.insert(k, Monkey::Op(w[0].to_string(),
                                     w[2].to_string(),
                                     w[1].to_string().chars().nth(0).unwrap()));
        }
    });

    return map;
}

fn solve(monkeys: &Input, root: String, humn: Option<i128>) -> i128 {
    if let Some(x) = humn {
        if root == "humn" {
            return x;
        }
    }
    if let Some(monke) = monkeys.get(&root) {
        return match monke {
            Monkey::Num(x) => *x,
            Monkey::Op(arg1, arg2, op) => {
                let a1 = arg1.to_string();
                let a2 = arg2.to_string();
                match op {
                    '+' =>  solve(monkeys, a1, humn) + solve(monkeys, a2, humn),
                    '*' =>  solve(monkeys, a1, humn) * solve(monkeys, a2, humn),
                    '/' =>  solve(monkeys, a1, humn) / solve(monkeys, a2, humn),
                    '-' =>  solve(monkeys, a1, humn) - solve(monkeys, a2, humn),
                    _x => panic!("BAD OP")
                }
            }
        }
    } else {
        panic!("NO MONKE");
    }
}

pub fn part1(input: Input) -> i128 {
    return solve(&input, "root".to_string(), None);
}

pub fn part2(input: Input) -> i128 {
    let r = input.get(&"root".to_string()).unwrap();
    if let Monkey::Op(p1, p2, _op) = r {

        let t = solve(&input, p2.to_string(), Some(0));
        let mut low : f64 = 0.0;
        let mut hih : f64 = 1e25;
        while low < hih {
            let mid = ((low+hih)/2.0).floor();
            let s = t - solve(&input, p1.to_string(), Some(mid as i128));
            if s < 0 {
                low = mid;
            } else if s == 0 {
                return mid as i128;
            } else {
                hih = mid;
            }
        }
    } else {
        panic!("WHY AM I REQUIRED TO DO THIS RUST?!?!?!");
    }
    return 0;
}
