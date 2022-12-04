            fn validate(s: &str) -> bool {
    let mut kn = 0;
    let keys = s.split_whitespace();
    for l in keys {
        let line = l.trim_end().trim_start();
        let (k, v) = line.split_once(':').unwrap();
        match k {
            "byr" => {
                if let Ok(year) = v.parse::<u32>() {
                    kn += (year >= 1920 && year <= 2002) as u32;
                } else {
                    return false;
                }
            },
            "iyr" => {
                if let Ok(year) = v.parse::<u32>() {
                    kn += (year >= 2010 && year <= 2020) as u32;
                } else {
                    return false;
                }
            },
            "eyr" => {
                if let Ok(year) = v.parse::<u32>() {
                    kn += (year >= 2020 && year <= 2030) as u32;
                } else {
                    return false;
                }
            },
            "hgt" => {
                if v.contains("cm") {
                    if let Ok(height) = v.trim_end_matches("cm").parse::<u32>() {
                        kn += (height >= 150 && height <= 193) as u32;
                    } else {
                        return false;
                    }
                } else if v.contains("in") {
                    if let Ok(height) = v.trim_end_matches("in").parse::<u32>() {
                        kn += (height >= 59 && height <= 76) as u32;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            "hcl" => {
                if v.chars().nth(0).unwrap().eq(&'#') {
                    let hex = v.trim_start_matches('#');
                    if hex.len() == 6 {
                        for c in hex.chars() {
                            if !"0123456789abcdef".contains(c) {
                                return false;
                            }
                        }
                        kn += 1;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            "ecl" => {
                let mut found = false;
                for c in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] {
                    if c.eq(v) {
                        found = true;
                        break;
                    }
                }
                if found {
                    kn += 1;
                } else {
                    return false;
                }
            },
            "pid" => {
                if v.len() == 9 && v.parse::<u64>().is_ok() {
                    kn += 1;
                } else {
                    return false;
                }
            },
            "cid" => {},
            _ => panic!("UNKNOWN KEY")
        }
    }

    return kn >= 7;
}


fn main() {
    println!("{}",
        include_str!("../input.txt")
        .split("\n\n")
        .filter(|l| validate(l))
        .count());

}
