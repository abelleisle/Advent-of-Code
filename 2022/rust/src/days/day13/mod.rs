use std::cmp::Ordering;

#[derive(Eq, Clone)]
pub enum Node{
    Number(isize),
    List(Vec<Node>)
}

impl Node {
    fn new(input: &str) -> Self {
        // List
        if input.starts_with('[') {
            let mut values = Vec::new();
            let input = input
                .strip_prefix('[').unwrap()
                .strip_suffix(']').unwrap();
            let mut depth = 0;
            let mut tp = String::new();
            for c in input.chars() {
                match c {
                    '[' => {
                        depth += 1;
                        tp.push('[');
                    },
                    ']' => {
                        depth -= 1;
                        tp.push(']');
                    },
                    ',' => {
                        if depth == 0 {
                            values.push(Node::new(&tp));
                            tp.clear();
                        } else {
                            tp.push(',');
                        }
                    },
                    x => tp.push(x)
                }
            }

            if tp.len() > 0 {
                values.push(Node::new(&tp));
            }

            return Node::List(values);
        } else {
            return Node::Number(input.parse().unwrap());
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (    Node::Number(l),     Node::Number(r)) => l.cmp(r),
            (    Node::List(l)  ,     Node::List(r)  ) => l.cmp(r),
            (l @ Node::Number(_),     Node::List(r)  ) => std::slice::from_ref(l).cmp(r.as_slice()),
            (    Node::List(l)  , r @ Node::Number(_)) => l.as_slice().cmp(std::slice::from_ref(r)),
        }
    }
}

type Input = Vec<(Node, Node)>;

pub fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| l.split_once('\n').unwrap())
        .map(|(l,r)| {
            (Node::new(l.trim_end()), Node::new(r.trim_end()))
        }).collect()
}

pub fn part1(input: Input) -> usize {
    return input
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p.0.cmp(&p.1) == Ordering::Less {
            true  => Some(i + 1),
            false => None
        })
        .sum();
}

pub fn part2(input: Input) -> usize {
    let mut p = input.iter()
        .map(|l| vec![l.0.clone(), l.1.clone()])
        .collect::<Vec<Vec<Node>>>()
        .concat();

    p.extend_from_slice(&vec![Node::new("[[2]]"), Node::new("[[6]]")]);
    p.sort_unstable();

    if let Ok(a) = p.binary_search(&Node::Number(2)) {
        if let Ok(b) = p.binary_search(&Node::Number(6)) {
            return (a + 1) * (b + 1);
        }
    }

    return 0;
}
