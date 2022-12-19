use std::collections::{HashSet, HashMap};

type Input = Vec<char>;

pub fn parse(input: &str) -> Input {
    return input.trim_end().chars().map(|c| c).collect::<Input>().clone();
}

type Rock = HashSet<(isize, isize)>;

fn get_rock(t: usize, y: isize) -> Rock {
    match t {
        0 => HashSet::from([(2,y), (3,y), (4,y), (5,y)]), // Horizontal Line
        1 => HashSet::from([(3,y+2), (2,y+1), (3,y+1), (4,y+1), (3,y)]), // Cross
        2 => HashSet::from([(2,y), (3,y), (4,y), (4,y+1), (4,y+2)]), // L shape
        3 => HashSet::from([(2,y), (2,y+1), (2,y+2), (2,y+3)]), // Vertical Line
        4 => HashSet::from([(2,y), (2,y+1), (3,y), (3,y+1)]), // 2x2 Box
        _ => panic!("Unknown rock type {t}!")
    }
}

fn rock_left(r: &mut Rock) {
    if !r.iter().any(|c| c.0 == 0) {
        *r = r.iter().map(|c| (c.0-1, c.1)).collect::<Rock>();
    }
}

fn rock_right(r: &mut Rock) {
    if !r.iter().any(|c| c.0 == 6) {
        *r = r.iter().map(|c| (c.0+1, c.1)).collect::<Rock>();
    }
}

fn rock_down(r: &mut Rock) {
    *r = r.iter().map(|c| (c.0, c.1-1)).collect::<Rock>();
}

fn rock_up(r: &mut Rock) {
    *r = r.iter().map(|c| (c.0, c.1+1)).collect::<Rock>();
}

fn top_sig(rlist: &Rock, h: isize) -> u64 {
    let mut sig : u64 = 0;
    for y in h-5..=h {
        sig <<= 3;
        sig += rlist.iter().filter(|l| l.1 == y).count() as u64;
    }
    return sig;
}

fn drop_rocks(rlist: &mut Rock, dirs: &Input, amount: usize) -> isize {
    // k = t%5, d, [rlist top 20 sig]
    // v = t, h
    let mut cache : HashMap<(usize, usize, u64), (usize, isize)>
     = HashMap::new();
    let mut h : isize = 0;
    let mut ah : isize = 0;
    let mut d = 0;
    let mut t = 0;
    while t < amount {
        let mut r = get_rock(t%5, h+4);
        // Keep dropping the rock
        loop {
            /* Perform push */
            match dirs[d] {
                '<' => { // Left
                    rock_left(&mut r);
                    if !r.is_disjoint(rlist) {
                        rock_right(&mut r);
                    }
                },
                '>' => { // Right
                    rock_right(&mut r);
                    if !r.is_disjoint(rlist) {
                        rock_left(&mut r);
                    }
                },
                _ => panic!("Unknown direction")
            };
            d = (d+1)%dirs.len();

            /* Move it down */
            rock_down(&mut r);
            /* Hit the bottom */
            if !r.is_disjoint(rlist) {
                rock_up(&mut r);
                r.iter().for_each(|p| {rlist.insert(*p);});
                h = h.max(r.iter().max_by(|p, q| p.1.cmp(&q.1)).unwrap().1);
                /* Only cache if greater than input size.
                 * A repetition can only exist after a full loop. */
                if amount >= dirs.len() {
                    let sig = (t%5, d, top_sig(rlist, h));
                    if let Some(old) = cache.get(&sig) {
                        let (ot, oh) = (old.0, old.1);
                        let dh = h-oh;
                        let dt = t-ot;
                        let jump = (amount-t)/dt;
                        ah += (jump as isize)*dh;
                        t += jump*dt;
                        assert!(t <= amount);
                    }
                    cache.insert(sig,(t,h));
                }
                break;
            }
        }
        t += 1;
    }

    return h+ah;
}

pub fn part1(input: Input) -> isize {
    let mut rocks : Rock = HashSet::from([(0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0)]);
    return drop_rocks(&mut rocks, &input, 2022);
}

pub fn part2(input: Input) -> isize {
    let mut rocks : Rock = HashSet::from([(0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0)]);
    return drop_rocks(&mut rocks, &input, 1000000000000);
}
