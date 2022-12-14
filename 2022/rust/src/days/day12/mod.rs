use std::collections::{VecDeque, HashSet};

type Input = Vec<Vec<char>>;

pub fn parse(input: &str) -> Input {
    let f : Vec<&str> = input.lines().collect();
    let mut land : Input = Vec::new();
    for l in f.iter() {
        land.push(Vec::new());
        for c in l.chars() {
            let size = land.len() - 1;
            land[size].push(c);
        }
    }
    return land;
}

fn find(land: &Input, target: char) -> Option<(isize, isize)> {
    for (y,l) in land.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if c == &target {
                return Some((x as isize, y as isize));
            }
        }
    }
    return None;
}

fn char_at(land: &Input, loc: (isize, isize)) -> Option<char> {
    if loc.0 >= 0 && loc.0 < land[0].len() as isize &&
       loc.1 >= 0 && loc.1 < land.len() as isize {
        return Some(land[loc.1 as usize][loc.0 as usize]);
    }

    return None;
}

fn at(land: &Input, loc: (isize, isize)) -> Option<isize> {
    if let Some(a) = char_at(&land, loc) {
        let c = match a {
            'E' => 26,
            'S' => 1,
            x => (x as isize) - ('a' as isize)
        };
        return Some(c);
    }

    return None;
}

fn bfs(land: &Input, start: (isize, isize)) -> Option<isize> {
    let mut q : VecDeque<((isize, isize), isize)> = VecDeque::new();
    q.push_back(((start.0, start.1), 0));

    let mut v : HashSet<(isize, isize)> = HashSet::new();
    while q.len() > 0 {
        let ((x,y), d) = q.pop_front().unwrap();
        if v.contains(&(x,y)) { // Already visited, don't visit again
            continue;
        }
        v.insert((x,y)); // Visit it
        if char_at(&land, (x,y)).unwrap() == 'E' {
            return Some(d - 2);
        }
        for (dx, dy) in [(0,1), (0,-1), (1,0), (-1,0)] {
            let xx = x + dx;
            let yy = y + dy;
            if let Some(h) = at(&land, (xx, yy)) {
                if let Some(t) = at(&land, (x, y)) {
                    if h - t <= 1 {
                        q.push_back(((xx, yy), d + 1));
                    }
                }
            }
        }

    }

    return None;
}

pub fn part1(input: Input) -> isize {
    let start = find(&input, 'S').unwrap();

    return bfs(&input, start).unwrap();
}

pub fn part2(input: Input) -> isize {
    let mut v : Vec<isize> = Vec::new();
    for (y,l) in input.iter().enumerate() {
        for (x,c) in l.iter().enumerate() {
            if c == &'a' || c == &'S' {
                if let Some(len) = bfs(&input, (x as isize, y as isize)) {
                    v.push(len);
                }
            }
        }
    }

    return *v.iter().min().unwrap();
}
