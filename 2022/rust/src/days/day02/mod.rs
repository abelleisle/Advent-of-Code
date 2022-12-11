type Input<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Input {
    return input.lines().collect::<Input>().clone();
}

pub fn part1(input: Input) -> usize {
    let sum = input.iter()
        .map(|l| {
            match l {
                &"A X" => 1 + 3,
                &"B X" => 1 + 0,
                &"C X" => 1 + 6,
                &"A Y" => 2 + 6,
                &"B Y" => 2 + 3,
                &"C Y" => 2 + 0,
                &"A Z" => 3 + 0,
                &"B Z" => 3 + 6,
                &"C Z" => 3 + 3,
                _ => 0
            }
        }).sum();
    return sum;
}

pub fn part2(input: Input) -> usize {
    let sum = input.iter()
        .map(|l| {
            match l {
                &"A X" => 0 + 3,
                &"B X" => 0 + 1,
                &"C X" => 0 + 2,
                &"A Y" => 3 + 1,
                &"B Y" => 3 + 2,
                &"C Y" => 3 + 3,
                &"A Z" => 6 + 2,
                &"B Z" => 6 + 3,
                &"C Z" => 6 + 1,
                _ => 0
            }
        }).sum();
    return sum;
}
