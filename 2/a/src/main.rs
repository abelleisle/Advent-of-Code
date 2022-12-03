use itertools::Itertools;

fn valid((min, max) : (u32, u32), c: char, p: &str) -> bool {
    let num = p.chars().filter(|d| d.eq(&c)).count() as u32;
    return num >= min && num <= max;
}

fn main() {
    let valid : usize =
        include_str!("../input.txt")
        .lines()
        .filter(|l| {
            let s = l.split_whitespace().collect::<Vec<_>>();
            let (min,max) = s[0].split_once('-').unwrap();
            let c = s[1].trim_end_matches(':').chars().nth(0).unwrap();
            let pwd = s[2];
            return valid((min.parse().unwrap(),max.parse().unwrap()),c,pwd);
        })
        .count();
    println!("{}", valid);
}

/*
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
*/
