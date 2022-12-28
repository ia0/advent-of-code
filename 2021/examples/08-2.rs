use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn set<T>(x: &mut Option<T>, v: T) {
    assert!(x.is_none());
    *x = Some(v);
}

const DIGITS: [(usize, &'static [usize]); 10] = [
    (0, &[0, 1, 2, 4, 5, 6]),
    (1, &[2, 5]),
    (2, &[0, 2, 3, 4, 6]),
    (3, &[0, 2, 3, 5, 6]),
    (4, &[1, 2, 3, 5]),
    (5, &[0, 1, 3, 5, 6]),
    (6, &[0, 1, 3, 4, 5, 6]),
    (7, &[0, 2, 5]),
    (8, &[0, 1, 2, 3, 4, 5, 6]),
    (9, &[0, 1, 2, 3, 5, 6]),
];

#[derive(Default, Debug)]
struct Problem {
    patterns: [HashSet<u8>; 10],
    output: [HashSet<u8>; 4],
}

impl Problem {
    fn parse(input: &str) -> Problem {
        let words: Vec<_> = input.split_whitespace().collect();
        assert_eq!(words.len(), 15);
        let mut problem = Problem::default();
        for i in 0 .. 10 {
            problem.patterns[i] = words[i].as_bytes().iter().cloned().collect();
        }
        for i in 0 .. 4 {
            problem.output[i] = words[11 + i].as_bytes().iter().cloned().collect();
        }
        problem
    }

    fn solve(&self) -> usize {
        let mut count: HashMap<u8, usize> = HashMap::new();
        let mut one = None;
        let mut four = None;
        for p in &self.patterns {
            for &x in p {
                *count.entry(x).or_default() += 1;
            }
            match p.len() {
                2 => set(&mut one, p),
                4 => set(&mut four, p),
                _ => (),
            }
        }
        let mut to_fake = [None; 7];
        for (fake, c) in count {
            let real = match c {
                4 => 4,
                6 => 1,
                7 if four.unwrap().contains(&fake) => 3,
                7 => 6,
                8 if one.unwrap().contains(&fake) => 2,
                8 => 0,
                9 => 5,
                _ => unreachable!(),
            };
            set(&mut to_fake[real], fake);
        }
        let mut digits = HashMap::new();
        for (digit, reals) in DIGITS {
            let mut z: Vec<u8> = reals.iter().map(|&x| to_fake[x].unwrap()).collect();
            z.sort();
            assert!(digits.insert(z, digit).is_none());
        }
        let mut r = 0;
        for fakes in &self.output {
            let mut z: Vec<_> = fakes.iter().cloned().collect();
            z.sort();
            r = 10 * r + digits[&z];
        }
        r
    }
}

fn main() {
    let input = File::open("examples/08.txt").unwrap();
    let problems: Vec<_> =
        BufReader::new(input).lines().map(|x| Problem::parse(&x.unwrap())).collect();
    println!("{}", problems.iter().map(|p| p.solve()).sum::<usize>());
}
