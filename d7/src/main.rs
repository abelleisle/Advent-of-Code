#![allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {

    let mut size : HashMap<String, usize> = HashMap::new();
    let mut path : Vec<&str> = Vec::new();

    include_str!("../input.txt")
        .lines()
        .for_each(|l| {
            let seg : Vec<&str> = l.trim_end().split(' ').collect();
            match seg[1] {
                "cd" => {
                    match seg[2] {
                        ".." => {_ = path.pop()},
                        &_ => path.push(seg[2])

                    }
                },
                "ls" => {},
                &_ => {
                    if let Ok(sz) = seg[0].parse::<usize>() {
                        for p in 0..=path.len() {
                            let sl = &path[0..p].join("/");
                            if let Some(psize) = size.get_mut(sl) {
                                *psize += sz;
                            } else {
                                size.insert(sl.to_string(), sz);
                            }
                        }
                    }
                }
            }
        });

    let mut part1 = 0;
    let mut part2 = usize::MAX;

    let delete_required : usize = size.get("/").unwrap() - (70000000 - 30000000);

    for (_,v) in size.iter() {
        if *v <= 100_000 {
            part1 += *v;
        }

        if *v >= delete_required {
            part2 = part2.min(*v);
        }

    }

    println!("P1: {part1}");
    println!("P2: {part2}");
}
