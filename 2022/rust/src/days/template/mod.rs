type Input<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Input {
    return input.lines().clone();
}

pub fn part1(input: Input) -> isize {
    return input.len();
}

pub fn part2(input: Input) -> isize {
    return input.len() + 1;
}
