fn greatest_n(v: Vec<u32>, n: usize) -> Vec<u32>
{
    let mut greatest = v.to_vec();
    greatest.sort_by(|a, b| b.cmp(a));
    greatest.truncate(n.min(v.len()));
    return greatest;
}

fn main()
{
    let mut elves = Vec::new();

    include_str!("../input.txt")
        .lines()
        .for_each(|x| {
            match x.parse::<u32>() {
                Ok(y) => {
                    if let Some(last) = elves.last_mut() {
                        *last += y;
                    } else {
                        elves.push(0);
                    }
                },
                Err(_) => elves.push(0)
            }
        });

    println!("{}", greatest_n(elves, 3).iter().sum::<u32>());
}

