use crate::helper::nums_in_str;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Clone, PartialEq)]
pub enum Env {
    Stone,
    Floor,
    Sand,
}

type Input = HashMap<(isize, isize), Env>;

pub fn parse(input: &str) -> Input {
    let mut cave : Input = HashMap::new();

    input.lines()
        .for_each(|l| {
            let nums : Vec<(isize, isize)> = nums_in_str::<isize>(l)
                .iter()
                .tuples()
                .map(|(x,y)| (x.clone(),y.clone()))
                .collect();

            for (i, c) in nums.iter().enumerate() {
                if i < nums.len() - 1 {
                    // Same x
                    if c.0 == nums[i+1].0 {
                        for y in c.1.min(nums[i+1].1)..=c.1.max(nums[i+1].1) {
                            cave.insert((c.0, y), Env::Stone);
                        }
                    // Same y
                    } else if c.1 == nums[i+1].1 {
                        for x in c.0.min(nums[i+1].0)..=c.0.max(nums[i+1].0) {
                            cave.insert((x, c.1), Env::Stone);
                        }
                    }
                }
            }
        });

    let floor = cave.iter().map(|k| k.0.1).max().unwrap() + 2;
    for x in -2000..2000 {
        cave.insert((x,floor), Env::Floor);
    }

    return cave.clone();
}

pub fn draw(cave: &mut Input, sand: (isize, isize)) {
    let screen_height = 49;
    let (xs,xe) = (cave.iter().filter(|k| k.1.ne(&Env::Floor)).map(|k| k.0.0).min().unwrap() - 1,
                   cave.iter().filter(|k| k.1.ne(&Env::Floor)).map(|k| k.0.0).max().unwrap() + 1);
    let (mut ys,mut ye) = (0,
                           cave.iter().filter(|k| k.1.ne(&Env::Floor)).map(|k| k.0.1).max().unwrap() + 1);
    let ls = cave.iter().filter(|k| k.1.eq(&Env::Sand)).map(|k| k.0.1).max().unwrap_or(ys).max(sand.1);

    if ye - ys > screen_height {
        ye = ys + screen_height;
        if ls + 5 >= ye {
            ye = ls + 5;
            ys = ye - screen_height;
        }
    }

    let mut crt : String = String::new();
    crt.push_str("\x1b[2J");
    let mut draw_at = |pos: (isize, isize), c: char, a: u8| {
        crt.push_str(&format!("\x1b[{};{}H\x1b[{}m{}",
                              pos.1, pos.0, a, c).to_string());
    };

    let (tl, tr) = (xs,ys);
    for y in ys..=ye {
        for x in xs..=xe {
            if sand.eq(&(x,y)) {
                draw_at((x-tl,y-tr), ' ', 43);
            } else {
                if let Some(c) = cave.get(&(x,y)) {
                    match c {
                        Env::Stone => draw_at((x-tl,y-tr), '#', 47),
                        Env::Sand  => draw_at((x-tl,y-tr), ' ', 43),
                        _ => {}
                    }
                } else {
                }
            }
        }
    }
    crt.push_str("\x1b[0m");
    print!("{crt}");
}

pub fn sand_fall(cave: &mut Input, floor: isize, fall: bool, animate: bool, fps: u64) -> bool {
    let mut sand = (500, 0);
    let sleep_cycle = 1000/fps.max(1);

    loop {
        /* Do we want to animate this? */
        if animate {
            draw(cave, sand);
            std::thread::sleep(std::time::Duration::from_millis(sleep_cycle));
        }

        /* If we hit the floor, just stop and return */
        if sand.1 + 1 >= floor && !fall{
            return false;
        }

        /* Fall */
        if !cave.contains_key(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
        }

        /* Can't fall, check sides */
        /* Down 1 left 1 */
        else if !cave.contains_key(&(sand.0 - 1, sand.1 + 1)) {
            sand.1 += 1;
            sand.0 -= 1;
        }

        /* Down 1 right 1 */
        else if !cave.contains_key(&(sand.0 + 1, sand.1 + 1)) {
            sand.1 += 1;
            sand.0 += 1;
        }

        /* We are at a resting place */
        else {
            break;
        }
    }

    /* Sand is at rest, insert it */
    cave.insert((sand.0, sand.1), Env::Sand);

    /* Sand didn't fall. We are in steady state */
    if sand.eq(&(500,0)) {
        return false;
    }

    return true;
}

pub fn part1(mut input: Input) -> usize {
    let floor = input.iter().map(|k| k.0.1).max().unwrap();
    loop {
        if !sand_fall(&mut input, floor, false, true, 100) {
            break;
        }
    }

    return input.iter().filter(|k| k.1.eq(&Env::Sand)).count();
}

pub fn part2(mut input: Input) -> usize {
    let floor = input.iter().map(|k| k.0.1).max().unwrap();
    loop {
        if !sand_fall(&mut input, floor, true, false, 0) {
            break;
        }
    }

    return input.iter().filter(|k| k.1.eq(&Env::Sand)).count();
}
