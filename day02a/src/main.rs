fn main()
{
    let lines : u32
    = include_str!("../input.txt")
        .lines()
        .map(|l| {
            match l {
                "A X" => 1 + 3,
                "B X" => 1 + 0,
                "C X" => 1 + 6,
                "A Y" => 2 + 6,
                "B Y" => 2 + 3,
                "C Y" => 2 + 0,
                "A Z" => 3 + 0,
                "B Z" => 3 + 6,
                "C Z" => 3 + 3,
                _ => 0
            }
        }).sum();

    println!("{}", lines);
}

