use crate::helper::nums_on_line;

#[derive(Clone)]
pub enum Op {
    Add(isize),
    Mult(isize)
}

#[derive(Clone)]
pub struct Monkey {
    items : Vec<isize>,
    oper  : Op,
    test  : isize,
    testr : (usize, usize),
    inspect : isize
}

impl Monkey {
    fn new(lines: std::str::Lines) -> Monkey {
        let mut monkey = Monkey {
            items: Vec::new(),
            oper: Op::Add(0),
            test: 0,
            testr: (0,0),
            inspect : 0
        };

        for (i, l) in lines.enumerate() {
            let line = l.trim_start().trim_end();
            if i == 0 {
                continue;
            }

            let (instr, end) = line.split_once(':').unwrap();
            let vals = end.trim_end().trim_start();
            match instr {
                "Starting items" => {
                    monkey.items.extend(nums_on_line(vals, &[' ', ',']).iter());
                },
                "Operation" => {
                    let x = *nums_on_line(vals, &[' ']).last().unwrap_or(&-1);
                    if vals.find('*').is_some() {
                        monkey.oper = Op::Mult(x);
                    } else if vals.find('+').is_some() {
                        monkey.oper = Op::Add(x);
                    } else {
                        panic!("Invalid op!");
                    }
                },
                "Test" => {
                    let x = *nums_on_line(vals, &[' ']).last().unwrap_or(&-1);
                    monkey.test = x;
                },
                "If true" => {
                    let x = *nums_on_line(vals, &[' ']).last().unwrap_or(&0);
                    monkey.testr.0 = x;
                },
                "If false" => {
                    let x = *nums_on_line(vals, &[' ']).last().unwrap_or(&0);
                    monkey.testr.1 = x;
                },
                &_ => panic!("Invalid expr")
            }
        }

        return monkey.clone();
    }

    fn throw_to(self: &mut Self, w: isize) {
        self.items.push(w);
    }
}

pub fn monkey_worry(
    mut input    : Input,
        rounds   : usize,
        worry_div: impl Fn(isize) -> isize
) -> isize {
    for _ in 0..rounds {
        for i in 0..input.len() {
            while let Some(item) = input[i].items.first() {
                let mut worry : isize = match input[i].oper {
                    Op::Mult(x) => item * if x == -1 {&item} else {&x},
                    Op::Add(x)  => item + if x == -1 {&item} else {&x}
                };

                worry = worry_div(worry);

                let t = if worry % input[i].test == 0 {
                    input[i].testr.0
                } else {
                    input[i].testr.1
                };
                input[t].throw_to(worry);

                input[i].inspect += 1;
                input[i].items.remove(0);
            }
        }
    }

    input.sort_by(|m, w| m.inspect.cmp(&w.inspect));
    input.pop().unwrap().inspect * input.pop().unwrap().inspect
}

type Input = Vec<Monkey>;

pub fn parse(input: &str) -> Input {
    let mut monkeys : Vec<Monkey> = Vec::new();
    input.split("\n\n")
        .map(|m| m.lines())
        .for_each(|l| {
            monkeys.push(Monkey::new(l));
        });

    return monkeys.clone();
}

pub fn part1(input: Input) -> isize {
    monkey_worry(input, 20, |x| {x / 3})
}

pub fn part2(input: Input) -> isize {
    // Use LCM to mod worry to keep it from getting too large
    let modu: isize = input.iter().map(|m| m.test).product();
    monkey_worry(input, 10000, |x| {x % modu})
}
