pub fn nums_on_line<T: std::str::FromStr>(line: &str, pat: &[char]) -> Vec<T> {
    return line
        .split(pat)
        .filter_map(|l| l.parse::<T>().ok())
        .collect();
}

