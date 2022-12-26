#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::{HashSet, HashMap, VecDeque};
use std::ops::{Range,RangeInclusive};
use std::rc::Rc;
use crate::helper::{nums_in_str, first_num_in_str};
use itertools::Itertools;

#[derive(Clone)]
pub enum Dir {
    Go(isize),
    Turn(char),
}

type Input = (Vec<Vec<char>>, Vec<Dir>);

pub fn parse(input: &str) -> Input {
//     let input = r"        ...#
//         .#..
//         #...
//         ....
// ...#.......#
// ........#...
// ..#....#....
// ..........#.
//         ...#....
//         .....#..
//         .#......
//         ......#.
//
// 10R5L5R10L4R5L5";
    let mut output : Input = (vec![], vec![]);
    let (map, dirs) = input.split_once("\n\n").unwrap();

    // Find widest and pad it
    let longest = map.lines().map(|l| l.len()).max().unwrap();

    for y in map.lines() {
        let llen = y.len();
        output.0.push(y.chars().collect::<Vec<char>>());
        let l = output.0.last_mut().unwrap();
        for _ in llen..longest {
            l.push(' ');
        }
    }

    let mut buf : String = String::new();
    for c in dirs.trim_end().chars() {
        if c.is_digit(10) {
            buf.push(c);
        }

        if !c.is_digit(10) {
            if !buf.is_empty() {
                output.1.push(Dir::Go(buf.parse::<isize>().unwrap()));
                buf.clear();
            }
            output.1.push(Dir::Turn(c));
        }
    }
    if !buf.is_empty() {
        output.1.push(Dir::Go(buf.parse::<isize>().unwrap()));
        buf.clear();
    }

    return output.clone();
}

fn get (grid: &Input, pos: (isize, isize)) -> char {
    let mut newp = pos;
    newp.1 = newp.1.rem_euclid(grid.0.len() as isize);
    newp.0 = newp.0.rem_euclid(grid.0[newp.1 as usize].len() as isize);
    return grid.0[newp.1 as usize][newp.0 as usize];
}

/* CPOS
 *     \/ origin
 *  .. 10 20
 *  .. 11
 *  02 12
 *  03 ..
 *
 * 10 = 0
 * 20 = 1
 * 11 = 2
 * 12 = 3
 * 02 = 4
 * 03 = 5
 */

struct Portal {
    // ((x,y),width,horizontal?)
    p1: ((isize, isize), isize, bool),
    p2: ((isize, isize), isize, bool),
    // -1 to flip, 1 to not
    c_flip: isize,
    d_flip: isize,
}

impl Portal {
    fn travel(self: &Self, mut p: (isize, isize), mut d: (isize, isize))
        -> Option<((isize, isize), (isize, isize))>
    {
        let mut tele = false;
        let mut fr = 0;
        let mut to = 0;
        let mut offset = 0;
        for (i,po) in [self.p1, self.p2].iter().enumerate() {
            // Horizontal
            if po.2 {
                // In portal
                if (d == (0,-1) || d == (0,1)) && p.1 == po.0.1 && p.0 >= po.0.0 && p.0 < (po.0.0 + po.1) {
                    tele = true;
                    to = (i+1)%2;
                    fr = i;
                    offset = p.0 - po.0.0;
                    break;
                }
            // Vertical
            } else {
                // In portal
                if (d == (-1,0) || d == (1,0)) && p.0 == po.0.0 && p.1 >= po.0.1 && p.1 < (po.0.1 + po.1) {
                    tele = true;
                    to = (i+1)%2;
                    fr = i;
                    offset = p.1 - po.0.1;
                    break;
                }
            }
        }

        let tp = match to {
            0 => self.p1,
            1 => self.p2,
            _ => panic!("FUCK TO")
        };

        let fp = match fr {
            0 => self.p1,
            1 => self.p2,
            _ => panic!("FUCK FROM")
        };

        if tele {
            p = tp.0;
            if self.c_flip == -1 {
                // Horiz
                if tp.2 == true {
                    p.0 += tp.1 - offset - 1;
                // Vert
                } else {
                    p.1 += tp.1 - offset - 1;
                }
            } else {
                // Horiz
                if tp.2 == true {
                    p.0 += offset;
                // Vert
                } else {
                    p.1 += offset;
                }
            };

            if fp.2 != tp.2 {
                (d.0,d.1) = (d.1,d.0);
            }
            d = (d.0 * self.d_flip, d.1 * self.d_flip);

            p = (p.0 + d.0, p.1 + d.1);
            assert!(p.0 >= 0 && p.0 < 150 && p.1 >= 0 && p.1 < 200);
            return Some((p,d));
        } else {
            return None;
        }

    }
}

fn next_pos(grid: &Input, mut pos: (isize, isize), mut dir: (isize, isize), part2: bool) -> ((isize, isize), (isize, isize)) {
    if !part2 { // P1
        let mut goal = (pos.0 + dir.0, pos.1 + dir.1);
        while get(grid, goal) == ' ' {
            goal.0 += dir.0;
            goal.1 += dir.1;
        }
        goal.1 = goal.1.rem_euclid(grid.0.len() as isize);
        goal.0 = goal.0.rem_euclid(grid.0[goal.1 as usize].len() as isize);
        return (goal, dir);
    } else { // P2
        let portals : Vec<Portal> = vec![
            Portal{p1: (( 50,  -1), 50, true ), p2: (( -1, 150), 50, false), c_flip:  1, d_flip: -1},
            Portal{p1: ((100,  -1), 50, true ), p2: ((  0, 200), 50, true ), c_flip:  1, d_flip:  1},
            Portal{p1: (( 49,   0), 50, false), p2: (( -1, 100), 50, false), c_flip: -1, d_flip: -1},
            Portal{p1: (( 49,  50), 50, false), p2: ((  0,  99), 50, true ), c_flip:  1, d_flip: -1},
            Portal{p1: ((150,   0), 50, false), p2: ((100, 100), 50, false), c_flip: -1, d_flip: -1},
            Portal{p1: ((100,  50), 50, true ), p2: ((100,  50), 50, false), c_flip:  1, d_flip: -1},
            Portal{p1: (( 50, 150), 50, true ), p2: (( 50, 150), 50, false), c_flip:  1, d_flip: -1},
        ];

        let mut tele = false;
        let goal = (pos.0 + dir.0, pos.1 + dir.1);
        for p in portals.iter() {
            if let Some((tp,td)) = p.travel(goal,dir) {
                pos = tp;
                dir = td;
                tele = true;
                break;
            }
        }

        if !tele {
            pos = goal;
        }

        assert!(pos.0 >= 0 && pos.0 < 150 && pos.1 >= 0 && pos.1 < 200);

        pos.1 = pos.1.rem_euclid(grid.0.len() as isize);
        pos.0 = pos.0.rem_euclid(grid.0[pos.1 as usize].len() as isize);

        return (pos, dir);
    }
}

fn solve(input: &Input, part2: bool) -> isize {
    // for yc in input.0.iter() {
    //     for xc in yc.iter() {
    //         print!("{xc}");
    //     }
    //     println!("");
    // }

    let mut start : (isize, isize) = (0,0);
    let mut dir : (isize, isize) = (1,0);
    'outer: for (y, yc) in input.0.iter().enumerate() {
        for (x, xc) in yc.iter().enumerate() {
            if xc == &'.' {
                start = (x as isize, y as isize);
                break 'outer;
            }
        }
    }

    // println!("Start: {},{}", start.0, start.1);

    let mut pos = start;

    for d in input.1.iter() {
        let pdd = match dir {
            ( 1, 0) => 0,
            ( 0, 1) => 1,
            (-1, 0) => 2,
            ( 0,-1) => 3,
            _ => panic!("Shit")
        };
        match d {
            Dir::Go(x) => {
                // println!("step {} {} {} {}",pos.1+1,pos.0+1,pdd,x);
                'inner: for _a in 0..*x {
                    let np = next_pos(&input, pos, dir, part2);
                    // println!("{},{}",np.0.0,np.0.1);
                    let gc = get(&input, np.0);
                    assert!(gc != ' ');
                    if gc == '#' {
                        // println!("wall");
                        break 'inner;
                    } else {
                        pos = np.0;
                        dir = np.1;
                    }
                    // assert!(np.0.0 != 0 && np.0.1 != 149);
                    // println!("{},{}",pos.1+1,pos.0+1);
                }
            }
            Dir::Turn(c) => {
                // println!("step {} {} {} {}",pos.1+1,pos.0+1,pdd,c);
                match c {
                    'R' => {
                        dir = (-dir.1,  dir.0);
                    },
                    'L' => {
                        dir = ( dir.1, -dir.0);
                    },
                    _ => panic!("BAD GAME")
                }
            }
        }
    }

    let col = (1 + pos.0) * 4;
    let row = (1 + pos.1) * 1000;
    let ddd = match dir {
        ( 1, 0) => 0,
        ( 0, 1) => 1,
        (-1, 0) => 2,
        ( 0,-1) => 3,
        _ => panic!("Shit")
    };

    return col + row + ddd;
}

pub fn part1(input: Input) -> isize {
    return solve(&input, false);
}

pub fn part2(input: Input) -> isize {
    return solve(&input, true);
}
