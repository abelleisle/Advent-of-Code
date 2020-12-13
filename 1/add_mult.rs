#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::io::{BufRead, BufReader};

fn part1(f: std::fs::File) -> i32 {
    
    let nums: Vec<i32> = 
        BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    let len = nums.len();
    for i in 0..len-2 {
        for j in i..len-1 {
            if ((nums[i] + nums[j]) == 2020) {
                return nums[i] * nums[j];
            }
        }
    }

    return 0;
}

fn part2(f: std::fs::File) -> i32 {
    
    let nums: Vec<i32> = 
        BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    let len = nums.len();
    for i in 0..len-3 {
        for j in i..len-2 {
            for k in j..len-1 {
                if ((nums[i] + nums[j] + nums[k]) == 2020) {
                    return nums[i] * nums[j] * nums[k];
                }
            }
        }
    }

    return 0;
}

fn main() {
    let p1_result = part1(std::fs::File::open("input.txt").unwrap());
    println!("Part 1 Result: {}", p1_result);
    let p2_result = part2(std::fs::File::open("input.txt").unwrap());
    println!("Part 2 Result: {}", p2_result);
}
