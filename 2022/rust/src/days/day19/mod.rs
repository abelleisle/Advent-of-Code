use crate::helper::{nums_in_str, first_num_in_str};
use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
pub struct Blueprint {
    ore: isize,
    clay: isize,
    obby: (isize, isize),
    geode: (isize, isize)
}

impl Blueprint {
    fn new(s: &str) -> Self {
        let l = s.split('.').collect::<Vec<&str>>();
        let mut bp = Blueprint {
            ore: 0,
            clay: 0,
            obby: (0,0),
            geode: (0, 0)
        };
        bp.ore = first_num_in_str(l[0]).unwrap();
        bp.clay=  first_num_in_str(l[1]).unwrap();

        let obby = nums_in_str(l[2]);
        bp.obby = (obby[0], obby[1]);

        let geode = nums_in_str(l[3]);
        bp.geode = (geode[0], geode[1]);

        return bp;
    }
}

type Input = Vec<Blueprint>;

pub fn parse(input: &str) -> Input {
    let bps = input.lines()
        .map(|l| l.split_once(':').unwrap().1).map(|l| {
            Blueprint::new(l)
        }).collect::<Input>();
    return bps;
}

fn geode_max(bp: Blueprint, time: isize) -> isize {
    // Why have I done this to myself
    type Stack = (
        (isize, isize, isize, isize), // Robots: Ore, Clay, Obby, Geode
        (isize, isize, isize, isize), // Ore:    Ore, Clay, Obby, Geode
        isize                         // Time:   time...
    );
    let mut seen  : HashSet<Stack> = HashSet::new(); // Process this..
    let mut state : VecDeque<Stack> = VecDeque::from(
        [((1,0,0,0),(0,0,0,0), time)]
    );

    let mut max = 0;

    /* Something fun in store? */
    while let Some(mut s) = state.pop_front() {
        /* Make me life easier */
        let (
            (mut ro, mut rc, mut rb, rg),
            (mut o , mut c , mut b,  g),
            t
        ) = s;

        // Most geodes at once
        max = max.max(g);

        /* 1.21 Jiga-watts!?!?! */
        if t == 0 { continue; }

        /* Highest ore cost */
        let moc = *[bp.ore, bp.clay, bp.obby.0, bp.geode.0]
            .iter().max().unwrap() as isize;

        /* Track robot amounts */
        ro = ro.min(moc);        // Only need enough ore bots to gen max ore cost
        rc = rc.min(bp.obby.1);  // Only need enough clay bot for obby cost
        rb = rb.min(bp.geode.1); // Only need enough obby bots for geode cost
        /* How many ores do we want to shoot for?? */
        o = o.min(t*moc        - ro*(t-1)); // Ore Goal
        c = c.min(t*bp.obby.1  - rc*(t-1)); // Clay Goal
        b = b.min(t*bp.geode.1 - rb*(t-1)); // Obby Goal

        /* Save the modified state */
        s = ((ro,rc,rb,rg),(o,c,b,g),t);

        /* Deja-vu??? */
        if seen.contains(&s) { continue; }
        seen.insert(s);

        /* Next state if we buy nothing */
        state.push_back(((  ro,   rc,   rb,   rg),
                         (o+ro, c+rc, b+rb, g+rg),
                         t-1));
        /* State if we buy ore-bot */
        if o >= bp.ore {
            state.push_back(((         ro+1,   rc,   rb,   rg),
                             (o-bp.ore+ro  , c+rc, b+rb, g+rg),
                             t-1));
        }
        /* State if we buy clay-bot */
        if o >= bp.clay {
            state.push_back(((          ro,   rc+1,   rb,   rg),
                             (o-bp.clay+ro, c+rc  , b+rb, g+rg),
                             t-1));
        }
        /* State if we buy obby-bot */
        if o >= bp.obby.0 && c >= bp.obby.1 {
            state.push_back(((            ro,             rc,   rb+1,   rg),
                             (o-bp.obby.0+ro, c-bp.obby.1+rc, b+rb  , g+rg),
                             t-1));
        }
        /* State if we buy geode-bot */
        if o >= bp.geode.0 && b >= bp.geode.1 {
            state.push_back(((             ro,   rc,              rb,   rg+1),
                             (o-bp.geode.0+ro, c+rc, b-bp.geode.1+rb, g+rg  ),
                             t-1));
        }
    }

    return max;
}

pub fn part1(input: Input) -> isize {
    let mut p1 = 0;
    for (i, b) in input.iter().enumerate() {
        p1 += ((i+1) as isize)*geode_max(b.clone(), 24);
    }
    return p1;
}

pub fn part2(input: Input) -> isize {
    let mut p1 = 1;
    for i in 0..3 {
        p1 *= geode_max(input[i].clone(), 32);
    }
    return p1;
}
