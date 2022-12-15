pub fn nums_in_str<T: std::str::FromStr + std::fmt::Display>(line: &str) -> Vec<T> {
    let mut numbers : Vec<T> = Vec::new();

    if !line.chars().any(|c| c.is_digit(10)) {
        return numbers;
    }

    let mut in_number = false;
    let mut to_conv = false;
    let mut number = (0,0);

    for (i,c) in line.chars().enumerate() {
        let is_number = c.is_digit(10) || c.eq(&'-');
        // Char is number
        if is_number {
            if in_number == false {
                number.0 = i;
                in_number = true;
            }
        }

        // Char is not number
        if !is_number || i == line.len() - 1 {
            if in_number == true {
                number.1 = i;
                if i == line.len() - 1 {
                    number.1 += 1;
                }
                to_conv = true;
            }
            in_number = false;
        }


        if to_conv == true {
            if let Ok(x) = line[number.0..number.1].parse::<T>() {
                numbers.push(x);
            }
            to_conv = false;
        }
    }

    return numbers;
}

pub fn first_num_in_str<T: std::str::FromStr + std::fmt::Display>(line: &str) -> Option<T> {
    let mut in_number = false;
    let mut to_conv = false;
    let mut number = (0,0);

    if !line.chars().any(|c| c.is_digit(10)) {
        return None;
    }

    for (i,c) in line.chars().enumerate() {
        let is_number = c.is_digit(10);
        // Char is number
        if is_number {
            if in_number == false {
                number.0 = i;
                in_number = true;
            }
        }

        // Char is not number
        if !is_number || i == line.len() - 1 {
            if in_number == true {
                number.1 = i;
                if i == line.len() - 1 {
                    number.1 += 1;
                }
                to_conv = true;
            }
            in_number = false;
        }


        if to_conv == true {
            if let Ok(x) = line[number.0..number.1].parse::<T>() {
                return Some(x);
            }
            to_conv = false;
        }
    }

    return None;
}
