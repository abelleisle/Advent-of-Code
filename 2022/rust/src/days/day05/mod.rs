#![allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;

#[derive(Clone)]
pub struct Stack {
    boxes : Vec<char>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {boxes : Vec::new()}
    }

    pub fn insert(self : &mut Self, c : char) {
        self.boxes.push(c);
    }

    pub fn insert_str(self: &mut Self, s : &str) {
        for c in s.chars() {
            self.insert(c);
        }
    }

    pub fn pop(self: &mut Self) -> Option<char> {
        return self.boxes.pop();
    }

    pub fn top(self: &Self) -> Option<&char> {
        return self.boxes.last();
    }
}

type Input = (Vec<Stack>, String);

pub fn parse(input: &str) -> Input {
    let (stack,instr) = input
        .split_once("\n\n").unwrap();

    let mut stacks : Vec<Stack> = Vec::new();
    stack.lines()
        .map(|l| l.trim_end())
        .for_each(|l|{
            stacks.push(Stack::new());
            stacks.last_mut().unwrap().insert_str(l);
        });

    return (stacks, instr.to_string());
}

pub fn part1(input: Input) -> String {
    let (mut stacks, instr) = input;
    // move 6 from 4 to 3
    let re = Regex::new(r"move ([0-9]+) from ([0-9+]) to ([0-9]+)").unwrap();
    instr.lines()
        .for_each(|l| {
            for cap in re.captures_iter(l) {
                let (mov, fro, to) = (cap[1].parse::<usize>().unwrap(),
                                      cap[2].parse::<usize>().unwrap(),
                                      cap[3].parse::<usize>().unwrap());
                for _ in 0..mov {
                    let c = stacks[fro - 1].pop();
                    if let Some(cs) = c {
                        stacks[to - 1].insert(cs);
                    }
                }
            }

        });

    let mut ret = String::new();
    for n in stacks.iter() {
        let c = n.top();
        if c.is_some() {
            ret.push(*c.unwrap());
        }
    }

    return ret;
}

pub fn part2(input: Input) -> String {
    let (mut stacks, instr) = input;

    let re = Regex::new(r"move ([0-9]+) from ([0-9+]) to ([0-9]+)").unwrap();
    instr.lines()
        .for_each(|l| {
            for cap in re.captures_iter(l) {
                let (mov, fro, to) = (cap[1].parse::<usize>().unwrap(),
                                      cap[2].parse::<usize>().unwrap(),
                                      cap[3].parse::<usize>().unwrap());
                let mut v : Vec<char> = Vec::new();
                for _ in 0..mov {
                    let c = stacks[fro - 1].pop();
                    if let Some(cs) = c {
                        v.push(cs);
                    }
                }

                while let Some(vs) = v.pop() {
                    stacks[to - 1].insert(vs);
                }
            }

        });

    let mut ret = String::new();
    for n in stacks.iter() {
        let c = n.top();
        if c.is_some() {
            ret.push(*c.unwrap());
        }
    }

    return ret;
}
