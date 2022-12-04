fn main() {
    let highest_id =
        include_str!("../input.txt")
        .lines()
        .map(|l| l.split_at(7))
        .map(|(row, col)| {
            let mut r = 0;
            let mut c = 0;
            for w in row.chars() {
                r <<= 1;
                match w {
                    'B' => r += 1,
                    _ => {}
                }
            }


            for w in col.chars() {
                c <<= 1;
                match w {
                    'R' => c += 1,
                    _ => {}
                }
            }

            return r * 8 + c;
        }).max().unwrap();

    println!("{}", highest_id);
}
