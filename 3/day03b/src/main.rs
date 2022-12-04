use itertools::Itertools;

fn get_score(c : char) -> u32 {
    match c {
        'a'..='z' => return (c as u32 - 'a' as u32) + 1,
        'A'..='Z' => return (c as u32 - 'A' as u32) + 27,
        c => panic!{"{c}"}
    }
}

fn main()
{
    let input : u32 = include_str!("../input.txt")
        .lines()
        .tuples()
        .map(|(a,b,c)| {
            let mut score = 0;
            for d in c.chars() {
                if a.contains(d) && b.contains(d) {
                    score += get_score(d);
                    break;
                }
            }
            score
        })
        .sum();


    println!("{}", input);
}
