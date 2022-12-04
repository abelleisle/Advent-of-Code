fn main() {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    println!("{}",
    include_str!("../input.txt")
    .split("\n\n")
    .filter(|l| {
        for k in keys.iter() {
            if !l.contains(k) {
                return false
            }
        }
        true
    })
    .count());

}
