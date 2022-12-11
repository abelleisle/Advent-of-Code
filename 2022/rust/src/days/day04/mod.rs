type Input = Vec<(i64, i64, i64, i64)>;

pub fn parse(input: &str) -> Input {
    return input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(f,s)| {
            let (fss,fes) = f.split_once('-').unwrap();
            let (sss,ses) = s.split_once('-').unwrap();
            let fs : i64 = fss.parse().unwrap();
            let fe : i64 = fes.parse().unwrap();
            let ss : i64 = sss.parse().unwrap();
            let se : i64 = ses.parse().unwrap();
            (fs, fe, ss, se)
        }).collect();
}

pub fn part1(input: Input) -> usize {
    return input.iter().filter(|(fs,fe,ss,se)| {
        return (fs <= ss && fe >= se) || (ss <= fs && se >= fe)
    }).count();
}

pub fn part2(input: Input) -> usize {
    return input.iter().filter(|(fs,fe,ss,se)| {
        return fs.max(ss) <= fe.min(se);
    }).count();
}
