type Input<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Input {
    return input.lines().collect::<Input>().clone();
}

pub fn part1(input: Input) -> usize {
    return input.len();
}

pub fn part2(input: Input) -> usize {
    return input.len() + 1;
}
