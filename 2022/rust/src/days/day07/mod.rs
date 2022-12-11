use std::collections::HashMap;

type Input = HashMap<String, usize>;

pub fn parse(input: &str) -> Input {
    let mut size : HashMap<String, usize> = HashMap::new();
    let mut path : Vec<&str> = Vec::new();

    input.lines()
        .for_each(|l| {
            let seg : Vec<&str> = l.trim_end().split(' ').collect();
            match seg[1] {
                "cd" => {
                    match seg[2] {
                        ".." => {_ = path.pop()},
                        &_ => path.push(seg[2])

                    }
                },
                "ls" => {},
                &_ => {
                    if let Ok(sz) = seg[0].parse::<usize>() {
                        for p in 0..=path.len() {
                            let sl = &path[0..p].join("/");
                            if let Some(psize) = size.get_mut(sl) {
                                *psize += sz;
                            } else {
                                size.insert(sl.to_string(), sz);
                            }
                        }
                    }
                }
            }
        });

    return size;
}

pub fn part1(input: Input) -> usize {
    let mut part1 = 0;
    for (_,v) in input.iter() {
        if *v <= 100_000 {
            part1 += *v;
        }
    }

    return part1;
}

pub fn part2(input: Input) -> usize {
    let mut part2 = usize::MAX;

    let delete_required : usize = input.get("/").unwrap() - (70000000 - 30000000);

    for (_,v) in input.iter() {
        if *v >= delete_required {
            part2 = part2.min(*v);
        }

    }

    return part2;
}
