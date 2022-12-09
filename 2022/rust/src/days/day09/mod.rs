use std::collections::HashSet;

type Input = Vec<(char, isize)>;

pub fn dist(t: (isize, isize), h: (isize, isize)) -> isize {
    let x = (t.0 - h.0).abs();
    let y = (t.1 - h.1).abs();
    return x.max(y);
}

pub fn part1(input: Input) -> isize {
    let mut hist : HashSet<(isize, isize)> = HashSet::new();
    hist.insert((0,0));

    let mut t : (isize, isize) = (0, 0);
    let mut h : (isize, isize) = (0, 0);

    for i in input.iter() {
        for _ in 0..i.1 {
            match i.0 {
                'U' => h.1 += 1,
                'D' => h.1 -= 1,
                'L' => h.0 -= 1,
                'R' => h.0 += 1,
                _ => panic!("Unknown dir: {}", i.0)
            }

            while dist(t, h) > 1 {
                if (t.0-h.0).abs() > 0 {
                    t.0 += if h.0 > t.0 {1} else {-1};
                }

                if (t.1-h.1).abs() > 0 {
                    t.1 += if h.1 > t.1 {1} else {-1};
                }

                hist.insert(t);
            }
        }
    }

    return hist.len() as isize;
}

pub fn part2(input: Input) -> isize {
    let mut hist : HashSet<(isize, isize)> = HashSet::new();
    hist.insert((0,0));

    let mut k : Vec<(isize, isize)> = Vec::new();
    for _ in 0..10 {
        k.push((0,0));
    }

    for i in input.iter() {
        for _ in 0..i.1 {
            let mut h = *k.first().unwrap();
            match i.0 {
                'U' => h.1 += 1,
                'D' => h.1 -= 1,
                'L' => h.0 -= 1,
                'R' => h.0 += 1,
                _ => panic!("Unknown dir: {}", i.0)
            }
            *k.first_mut().unwrap() = h;

            for kn in 1..k.len() {
                let parent = k[kn-1];
                let mut knot = k[kn];
                while dist(parent, knot) > 1 {
                    if (knot.0-parent.0).abs() > 0 {
                        knot.0 += if parent.0 > knot.0 {1} else {-1};
                    }

                    if (knot.1-parent.1).abs() > 0 {
                        knot.1 += if parent.1 > knot.1 {1} else {-1};
                    }
                }
                k[kn] = knot;
            }
            hist.insert(*k.last().unwrap());
        }
    }

    return hist.len() as isize;
}

pub fn parse(input: &str) -> Input {
    return input
        .lines()
        .map(|l| {
            let n = l.split_once(' ').unwrap();
            let a = n.1.parse::<isize>().unwrap();
            (n.0.chars().nth(0).unwrap(), a)
        })
        .collect();
}
