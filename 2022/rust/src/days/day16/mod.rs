use std::collections::{HashMap, VecDeque};
use crate::helper::first_num_in_str;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Valve {
    ident: String,
    flow_rate: isize,
    leads_to:  Vec<String>
}

impl Valve {
    pub fn new(str: &str) -> Self {
        let mut s = Valve {
            ident: String::new(),
            flow_rate: 0,
            leads_to: Vec::new()
        };

        let split = str.split_once(';').unwrap();
        s.flow_rate = first_num_in_str(split.0).unwrap();

        let first : Vec<&str> = split.0.split(' ').collect();
        s.ident = first[1].to_string();

        let second: Vec<&str> = split.1.splitn(6, ' ').collect();
        if let Some(last) = second.last() {
            let idents = last.split(',');
            for i in idents {
                s.leads_to.push(i.trim_start().trim_end().to_string());
            }
        }

        return s;
    }
}

type Input = HashMap<String, Valve>;

pub fn parse(input: &str) -> Input {
    let mut valves = HashMap::new();

    input.lines()
        .for_each(|l| {
            let v = Valve::new(l);
            let k = v.ident.clone();
            valves.insert(k, v);
        });

    return valves;
}

struct ValveList {
    names: Vec<u16>, // List of names
    flows: Vec<isize>, // Flow for each index
    conns: Vec<Vec<u16>>, // List of indexes that connect to a node
    dist: Vec<Vec<isize>>,
    memoized_dfs: HashMap<(usize, VecDeque<usize>, isize, bool), isize>,
    start: usize
}

impl ValveList {
    fn new(input: &Input) -> Self {
        let mut vs = Self {
            names: vec![],
            flows: vec![],
            conns: vec![],
            dist: vec![vec![99; input.len()]; input.len()],
            memoized_dfs: HashMap::new(),
            start: 0
        };

        fn to_ident(s: String) -> u16 {
            let ident : u16 = ((s.chars().nth(0).unwrap() as u16) << 8)
                            | ((s.chars().nth(1).unwrap() as u16));
            return ident;
        }

        for (i,v) in input.iter().enumerate() {
            let s = to_ident(v.1.ident.to_string());
            if s == 16705 { // AA
                vs.start = i;
            };
            vs.names.push(s);
            vs.flows.push(v.1.flow_rate);
            vs.conns.push(
                v.1.leads_to.iter()
                    .map(|c| to_ident(c.to_string()))
                    .collect::<Vec<u16>>()
            );
            vs.dist[i][i] = 0;
        }
        for i in 0..vs.names.len() {
            for c in vs.conns[i].iter() {
                vs.dist[i][vs.names.iter().position(|&r| r == *c).unwrap()] = 1;
            }
        }

        /* Floyd-Warshall */
        for k in 0..vs.dist.len() {
            for i in 0..vs.dist.len() {
                for j in 0..vs.dist.len() {
                    vs.dist[i][j] = vs.dist[i][j].min(vs.dist[i][k] + vs.dist[k][j])
                }
            }
        }

        return vs;
    }

    fn dfs<'a>(
        self: &'a mut Self,
        cur: usize, // Index
        flows: VecDeque<usize>, // Indexes
        time: isize, // Time
        e: bool
    ) -> isize {
        if let Some(max) = self.memoized_dfs.get(&(cur, flows.clone(), time, e)) {
            return *max;
        }
        let max = if flows.is_empty() {
            0
        } else {
            *(0..flows.len()).map(|i| {
                let mut vc = flows.clone();
                if let Some(r) = vc.remove(i) {
                    let trav = self.dist[cur][r];
                    if trav < time {
                        (self.flows[r] * (time - trav - 1))
                        + self.dfs(r, vc, time - trav - 1, e)
                    } else {
                        0
                    }
                } else {
                    0
                }
            }).collect::<Vec<isize>>().iter().max().unwrap()
        };

        self.memoized_dfs.insert((cur, flows.clone(), time, e), max);

        return max.max(
            if e {
                self.dfs(self.start, flows.clone(), 26, false)
            } else {
                0
            }
        );
    }
}

/* Memoized dfs */

pub fn part1(input: Input) -> isize {
    let mut v = ValveList::new(&input);
    let flows = v.flows.iter()
        .enumerate()
        .filter(|(_,&f)| f != 0)
        .map(|(i,_)| i)
        .collect::<VecDeque<usize>>();
    return v.dfs(v.start, flows, 30, false);
}

pub fn part2(input: Input) -> isize {
    let mut v = ValveList::new(&input);
    let flows = v.flows.iter()
        .enumerate()
        .filter(|(_,&f)| f != 0)
        .map(|(i,_)| i)
        .collect::<VecDeque<usize>>();
    return v.dfs(v.start, flows, 26, true);
}
