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
                    "X" => { // Rock
                        score += 1;
                        match char {
                            "A" => score += 3,
                            "B" => score += 0,
                            "C" => score += 6,
                            _ => score += 0
                        }
                    },
                    "Y" => { // Paper
                        score += 2;
                        match char {
                            "A" => score += 6, // Rock
                            "B" => score += 3, // Paper
                            "C" => score += 0, // Scissors
                            _ => score += 0
                        }
                    },
                    "Z" => { // Scissors
                        score += 3;
                        match char {
                            "A" => score += 0, // Rock
                            "B" => score += 6,
                            "C" => score += 3,
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

