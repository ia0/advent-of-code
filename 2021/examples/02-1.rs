use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/02.txt").unwrap();
    let mut position: i64 = 0;
    let mut depth: i64 = 0;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split_whitespace().collect();
        assert_eq!(words.len(), 2);
        let amount: i64 = words[1].parse().unwrap();
        match words[0] {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => unreachable!(),
        }
    }
    println!("{}", position * depth);
}
