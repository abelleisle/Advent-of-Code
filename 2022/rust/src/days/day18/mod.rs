use crate::helper::nums_in_str;
use std::collections::{HashSet, VecDeque};

type Input = HashSet<(i32,i32,i32)>;

pub fn parse(input: &str) -> Input {
    return input
        .lines()
        .map(|l| {
            let nums = nums_in_str(l);
            (nums[0], nums[1], nums[2])
        }).collect::<Input>();
}

pub fn part1(input: Input) -> usize {
    let mut sides = 0;
    for c in input.iter() {
        let (x,y,z) = c;
        for d in [(-1,0,0),(1,0,0),(0,1,0),(0,-1,0),(0,0,-1),(0,0,1)] {
            if !input.contains(&(x+d.0, y+d.1, z+d.2)) {
                sides += 1;
            }
        }
    }
    return sides;
}

pub fn part2(input: Input) -> usize {
    let sides = part1(input.clone());
    let mut no_cubes : HashSet<(i32, i32, i32)> = HashSet::new();
    let xs = input.iter().map(|x| x.0).collect::<Vec<_>>();
    let ys = input.iter().map(|y| y.1).collect::<Vec<_>>();
    let zs = input.iter().map(|z| z.2).collect::<Vec<_>>();
    let (mnx, mxx) = (xs.iter().min().unwrap_or(&0), xs.iter().max().unwrap_or(&25));
    let (mny, mxy) = (ys.iter().min().unwrap_or(&0), ys.iter().max().unwrap_or(&25));
    let (mnz, mxz) = (zs.iter().min().unwrap_or(&0), zs.iter().max().unwrap_or(&25));
    for x in mnx-1..mxx+2 {
        for y in mny-1..mxy+2 {
            for z in mnz-1..mxz+2 {
                if !input.contains(&(x,y,z)) {
                    no_cubes.insert((x,y,z));
                }
            }
        }
    }

    let mut q : VecDeque<(i32, i32, i32)> = VecDeque::from([(mnx-1, mny-1, mnz-1)]);
    while let Some(c) = q.pop_front() {
        if no_cubes.contains(&c) {
            no_cubes.remove(&c);
            for d in [(-1,0,0),(1,0,0),(0,1,0),(0,-1,0),(0,0,-1),(0,0,1)] {
                q.push_back((c.0 + d.0, c.1 + d.1, c.2 + d.2));
            }
        }
    }

    let mut inner_sides = 0;
    for c in no_cubes.iter() {
        for d in [(-1,0,0),(1,0,0),(0,1,0),(0,-1,0),(0,0,-1),(0,0,1)] {
            if input.contains(&(c.0+d.0, c.1+d.1, c.2+d.2)) {
                inner_sides += 1;
            }
        }
    }

    return sides - inner_sides;
}
