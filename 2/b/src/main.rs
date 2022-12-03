use itertools::Itertools;

fn valid((min, max) : (u32, u32), c: char, p: &str) -> bool {
    return p.chars().nth((min - 1) as usize).unwrap().eq(&c)
        ^ p.chars().nth((max - 1) as usize).unwrap().eq(&c)
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
