use itertools::Itertools;

fn main() {
    let slope : Vec<&str> =
        include_str!("../input.txt")
        .lines()
        .collect();

    let mut trees = 0;
    let mut x = 0;
    for y in slope.iter() {
        match y.chars().nth(x % y.len()).unwrap() {
            '#' => trees += 1,
            '.' => {},
            _ => panic!("Unknown char")
        }
        x += 3;
    }

    println!("{}", trees);
}
