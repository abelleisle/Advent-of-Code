
fn get_score(c : char) -> u32 {
    match c {
        'a'..='z' => return (c as u32 - 'a' as u32) + 1,
        'A'..='Z' => return (c as u32 - 'A' as u32) + 27,
        c => panic!{"{c}"}
    }
}

fn main()
{
    let score : u32 = include_str!("../input.txt")
        .lines()
        .map(|l| { l.split_at(l.len()/2)})
        .map(|(x,y)| {
            let mut s = 0;
            for c in x.chars() {
                if y.contains(c) {
                    s += get_score(c);
                    break;
                }
            }
            s
        })
        .sum();

    println!("{}", score);
}
