use itertools::Itertools;

fn main() {
    let slope : Vec<&str> =
        include_str!("../input.txt")
        .lines()
        .collect();

    let mut trees = 1;

    for (xa, ya) in [(1,1), (3,1), (5,1), (7,1), (1,2)] {
        let mut lt = 0;
        let (mut x, mut y) = (0,0);

        while y < slope.len() {
            let line = slope[y];
            match line.chars().nth(x % line.len()).unwrap() {
                '#' => lt += 1,
                '.' => {},
                _ => panic!("Unknown char")
            }
            x += xa;
            y += ya;
        }

        println!("{lt}");

        trees *= lt;
    }

    println!("{}", trees);
}
