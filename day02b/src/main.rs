/*
 * A = rock     - 1
 * B = paper    - 2
 * C = scissors - 3
 */
fn main()
{
    let lines : u32
    = include_str!("../input.txt")
        .lines()
        .map(|l| {
            match l {
                "A X" => 0 + 3,
                "B X" => 0 + 1,
                "C X" => 0 + 2,
                "A Y" => 3 + 1,
                "B Y" => 3 + 2,
                "C Y" => 3 + 3,
                "A Z" => 6 + 2,
                "B Z" => 6 + 3,
                "C Z" => 6 + 1,
                _ => 0
            }
        }).sum();

    println!("{}", lines);
}

