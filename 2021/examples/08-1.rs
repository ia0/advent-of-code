use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
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

    fn count(&self) -> usize {
        self.output
            .iter()
            .filter(|x| match x.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
    }
}

fn main() {
    let input = File::open("examples/08.txt").unwrap();
    let problems: Vec<_> =
        BufReader::new(input).lines().map(|x| Problem::parse(&x.unwrap())).collect();
    let count: usize = problems.iter().map(|p| p.count()).sum();
    println!("{}", count);
}
