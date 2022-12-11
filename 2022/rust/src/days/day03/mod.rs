use itertools::Itertools;

type Input<'a> = Vec<&'a str>;

fn get_score(c : char) -> u32 {
    match c {
        'a'..='z' => return (c as u32 - 'a' as u32) + 1,
        'A'..='Z' => return (c as u32 - 'A' as u32) + 27,
        c => panic!{"{c}"}
    }
}

pub fn parse(input: &str) -> Input {
    return input.lines().collect::<Input>().clone();
}

pub fn part1(input: Input) -> u32 {
    let score = input.iter()
        .map(|l| { l.split_at(l.len()/2)})
        .map(|(x,y)| {
            let mut s = 0;
            for c in x.chars() {
                if y.contains(c) {
                    s += get_score(c);
                    break;
                }
            }
            s
        })
        .sum();

    return score;
}

pub fn part2(input: Input) -> u32 {
    let score = input.iter()
        .tuples()
        .map(|(a,b,c)| {
            let mut score = 0;
            for d in c.chars() {
                if a.contains(d) && b.contains(d) {
                    score += get_score(d);
                    break;
                }
            }
            score
        })
        .sum();

    return score;
}
