use itertools::Itertools;
use crate::helper::nums_in_str;
use std::collections::{HashSet, BTreeSet};

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
    let sensors : Vec<_> = input.0.iter().sorted_by(|a,b| a.1.cmp(&b.1)).rev().collect();

    /* Check overlap. Uses signed distance fields concept */
    let inside_dist = |x: isize, y: isize| -> Option<isize> {
        let mut largest_dist = None;
        for ((sx, sy), sd) in sensors.iter() {
            let sdist : isize = (sx.abs_diff(x) + sy.abs_diff(y)) as isize;
            if sdist > *sd {
                continue;
            }
            let dfe = ((sd-sdist) / 2) + 1;
            if largest_dist.is_none() || largest_dist.unwrap() < dfe {
                largest_dist = Some(dfe);
            }
        }
        return largest_dist;
    };

    /* Check each sensor's range */
    for ((sx,sy),sd) in sensors.iter() {
        /* Check the edges of each sensor  */
        for (da, db) in [((0,1),(1,0)),((1,0),(0,-1)),((0,-1),(-1,0)),((-1,0),(0,1))] {
            /* Get corner coords */
            let (ax, ay) = (sx + da.0*(sd+1), sy + da.1*(sd+1));
            let (bx, by) = (sx + db.0*(sd+1), sy + db.1*(sd+1));
            /* Get corner deltas */
            let (dx, dy) = ((bx-ax).clamp(-1, 1), (by-ay).clamp(-1, 1));
            assert!(ax.abs_diff(bx) == ay.abs_diff(by)); // Make sure we're square
            let mut i : isize = 0;
            let dline : isize = (ax-bx).abs(); // Length of diagonal
            /* Scan along edge */
            while i <= dline {
                let (px, py) = (ax + i*dx, ay + i*dy);
                if px < 0 || px > 4000000 || py < 0 || py > 4000000 {
                    i += 1;
                    continue;
                }
                /* Find the largest overlap in another sensor's range. This will
                 * allow 'i' to jump forward by the largest overlap. This will
                 * reduce the scanning range. */
                let d = inside_dist(px, py);
                if d.is_none() {
                    return 400000*px + py;
                }
                i += d.unwrap();
            }
        }
    }

    return 0;
}
