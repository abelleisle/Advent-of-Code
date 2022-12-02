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
            let mut char = "_";
            let mut score = 0;
            l.split(" ")
            .for_each(|x| {
                match x {
                    "A"|"B"|"C" => char = x,
                    "X" => { // Lose
                        score += 0;
                        match char {
                            "A" => score += 3, // Rock
                            "B" => score += 1, // Paper
                            "C" => score += 2, // Scissors
                            _ => score += 0
                        }
                    },
                    "Y" => { // Draw
                        score += 3;
                        match char {
                            "A" => score += 1, // Rock
                            "B" => score += 2, // Paper
                            "C" => score += 3, // Scissors
                            _ => score += 0
                        }
                    },
                    "Z" => { // Win
                        score += 6;
                        match char {
                            "A" => score += 2, // Rock
                            "B" => score += 3, // Paper
                            "C" => score += 1, // Scissors
                            _ => score += 0
                        }
                    },
                    _ => {}
                }
            });
            return score
        }).sum();

    println!("{}", lines);
}

