use itertools::Itertools;
use crate::helper::nums_in_str;
use std::collections::HashSet;

type Pos = (isize, isize);
type Input = (Vec<(Pos, isize)>, HashSet<Pos>);

pub fn parse(input: &str) -> Input {
    let mut sensors : Vec<(Pos, isize)> = Vec::new();
    let mut beacons : HashSet<Pos> = HashSet::new();
    input.lines()
        .for_each(|l| {
            let n = nums_in_str::<isize>(l);
            let s = (n[0],n[1]);
            let b = (n[2],n[3]);
            let d = (s.0-b.0).abs() + (s.1-b.1).abs();
            sensors.push((s,d));
            beacons.insert(b);
        });

    return (sensors,beacons);
}

pub fn part1(input: Input) -> usize {
    let target = 2000000;
    /* Find how many beacons exist at the target y */
    let bat = input.1.iter().filter(|b| b.1.eq(&target)).count();

    let mut no_b : HashSet<isize> = HashSet::new();

    for ((x, y), d) in input.0.iter() {
        let dist = (y-target).abs();
        if dist <= *d {
            let r = (dist-*d).abs();
            for tx in x-r..=x+r {
                no_b.insert(tx);
            }
        }
    }

    return no_b.len() - bat;
}

pub fn part2(input: Input) -> isize {
    let pairs : Vec<_> = input.0.iter().sorted_by(|a,b| a.1.cmp(&b.1)).rev().collect();
    /* Try every y coord in range */
    for ty in 0..=4000000 {
        /* Try every x coord in range */

        let mut tx : isize = 0;
        'xscan: while tx <= 4000000 {
            /* While checking each coord, check for sensor intersection */
            for ((x,y),bdist) in pairs.iter() {
                let sdist : isize = (x.abs_diff(tx) + y.abs_diff(ty)) as isize;
                /* If these beacons overlap */
                if *bdist >= sdist {
                    let ydiff = y.abs_diff(ty) as isize;     // Get height diff
                    let r = ydiff.abs_diff(*bdist) as isize; // Get width
                    tx = x+r+1; // Skip scanning this sensor range
                    continue 'xscan;
                }
            }

            /* This coordinate isn't in sensor range */
            return 400000*tx + ty;
        }
    }

    return 0;
}
