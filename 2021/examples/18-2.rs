use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
enum Num {
    Regular(i64),
    Pair { left: Box<Num>, right: Box<Num> },
}

fn read(input: &mut &[u8]) -> u8 {
    let r = input[0];
    *input = &input[1 ..];
    r
}

fn parse(input: &mut &[u8]) -> Num {
    match read(input) {
        b'[' => {
            let left = Box::new(parse(input));
            assert_eq!(read(input), b',');
            let right = Box::new(parse(input));
            assert_eq!(read(input), b']');
            Num::Pair { left, right }
        }
        x @ b'0' ..= b'9' => Num::Regular((x - b'0') as i64),
        _ => unreachable!(),
    }
}

impl Num {
    fn magnitude(&self) -> i64 {
        match self {
            Num::Regular(x) => *x,
            Num::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn reduce(&mut self) {
        while self.explode(0).is_some() || self.split() {}
    }

    fn regular(&self) -> i64 {
        match self {
            Num::Regular(x) => *x,
            _ => unreachable!(),
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<i64>, Option<i64>)> {
        match self {
            Num::Regular(_) => None,
            Num::Pair { left, right }
                if depth >= 4
                    && matches!(**left, Num::Regular(_))
                    && matches!(**right, Num::Regular(_)) =>
            {
                let left = left.regular();
                let right = right.regular();
                *self = Num::Regular(0);
                Some((Some(left), Some(right)))
            }
            Num::Pair { left, right } => {
                let depth = depth + 1;
                match left.explode(depth) {
                    None => match right.explode(depth) {
                        Some((Some(x), y)) => {
                            left.add_right(x);
                            Some((None, y))
                        }
                        x => x,
                    },
                    Some((x, Some(y))) => {
                        right.add_left(y);
                        Some((x, None))
                    }
                    x => x,
                }
            }
        }
    }

    fn add_left(&mut self, x: i64) {
        match self {
            Num::Regular(y) => *y += x,
            Num::Pair { left, .. } => left.add_left(x),
        }
    }

    fn add_right(&mut self, x: i64) {
        match self {
            Num::Regular(y) => *y += x,
            Num::Pair { right, .. } => right.add_right(x),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Num::Regular(x) if *x > 9 => {
                let left = Box::new(Num::Regular(*x / 2));
                let right = Box::new(Num::Regular((*x + 1) / 2));
                *self = Num::Pair { left, right };
                true
            }
            Num::Regular(_) => false,
            Num::Pair { left, right } => left.split() || right.split(),
        }
    }
}

fn add(x: Num, y: Num) -> Num {
    let mut r = Num::Pair { left: Box::new(x), right: Box::new(y) };
    r.reduce();
    r
}

fn main() {
    let input = File::open("examples/18.txt").unwrap();
    let mut nums = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut input = line.as_bytes();
        nums.push(parse(&mut input));
        assert!(input.is_empty());
    }
    println!(
        "{}",
        (0 .. nums.len())
            .flat_map(|i| (0 .. nums.len()).map(move |j| (i, j)))
            .filter(|&(i, j)| i != j)
            .map(|(i, j)| add(nums[i].clone(), nums[j].clone()).magnitude())
            .max()
            .unwrap()
    );
}
