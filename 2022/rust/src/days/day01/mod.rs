type Input = Vec<u32>;

fn greatest_n(v: Vec<u32>, n: usize) -> Vec<u32>
{
    let mut greatest = v.to_vec();
    greatest.sort_by(|a, b| b.cmp(a));
    greatest.truncate(n.min(v.len()));
    return greatest;
}

pub fn parse(input: &str) -> Input {
    return input
        .split("\n\n")
        .map(|l|
            l.lines()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        ).collect();
}

pub fn part1(input: Input) -> u32 {
    return greatest_n(input, 1)[0];
}

pub fn part2(input: Input) -> u32 {
    return greatest_n(input, 3).iter().sum::<u32>();
}
