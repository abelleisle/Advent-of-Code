#[derive(Clone)]
pub enum Instr {
    Noop,
    Stall,
    Addx(isize)
}

type Input = Vec<Instr>;

pub fn parse(input: &str) -> Input {
    let mut output : Vec<Instr> = Vec::new();
    for l in input.lines() {
        match &l[0..4] {
            "noop" => output.push(Instr::Noop),
            "addx" => {
                let s = l.trim_end().split_once(' ').unwrap();
                output.push(Instr::Stall);
                output.push(Instr::Addx(s.1.parse::<isize>().unwrap()));
            }
            _ => panic!{"Unknown instruction"}
        }
    }
    return output;
}

pub fn part1(input: Input) -> isize {
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut sig : isize = 1;
    let mut output : isize = 0;

    let mut cc = 1;
    for c in input {
        sig += match c {
            Instr::Addx(x) => x,
            _ => 0
        };

        cc += 1;
        if cycles.contains(&cc) {
            output += sig * cc;
        }
    }

    return output;
}

pub fn part2(input: Input) -> String {
    let mut sig : isize = 1;

    let mut screen = String::new();
    screen.push('\n');

    for (cycle,c) in input.iter().enumerate() {
        let hpos = (cycle % 40) as isize;

        if sig - 1 <= hpos && hpos <= sig + 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        sig += match c {
            Instr::Addx(x) => *x as isize,
            _ => 0
        };

        if (cycle) % 40 == 39 {
            screen.push('\n');
        }
    }

    return screen;
}
