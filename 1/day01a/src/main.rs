fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|x| {
            match x.parse::<u32>() {
                Ok(y) => y,
                Err(_) => 0
            }
        })
        .collect::<Vec<u32>>();


    let mut elves = Vec::new();
    elves.push(0);

    for l in lines {
        if l > 0 {
            let len = elves.len();
            elves[len - 1] += l;
        } else {
            elves.push(0);
        }
    }

    let max : u32 = elves.into_iter().fold(0, std::cmp::max);
    println!("{}", max);
}
