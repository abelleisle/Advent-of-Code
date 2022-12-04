// use itertools::Itertools;
// use regex::Regex;

fn main() {
    let overlap = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .filter(|(f,s)| {
            let (fss,fes) = f.split_once('-').unwrap();
            let (sss,ses) = s.split_once('-').unwrap();
            let fs : i64 = fss.parse().unwrap();
            let fe : i64 = fes.parse().unwrap();
            let ss : i64 = sss.parse().unwrap();
            let se : i64 = ses.parse().unwrap();
            return fs.max(ss) <= fe.min(se);
        }).count();

    println!("{}", overlap);
}
