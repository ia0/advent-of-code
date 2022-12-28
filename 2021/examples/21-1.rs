use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let input = File::open("examples/21.txt").unwrap();
    let regex = Regex::new(r#"starting position: ([0-9]+)"#).unwrap();
    let mut positions: Vec<usize> = BufReader::new(input)
        .lines()
        .map(|line| regex.captures(&line.unwrap()).unwrap()[1].parse().unwrap())
        .collect();
    let mut scores = [0, 0];
    let mut current = 0;
    let mut count = 0;
    let mut dice = (1 ..= 100).cycle();
    loop {
        positions[current] += (0 .. 3).map(|_| dice.next().unwrap()).sum::<usize>();
        count += 3;
        positions[current] %= 10;
        if positions[current] == 0 {
            positions[current] = 10;
        }
        scores[current] += positions[current];
        if scores[current] >= 1000 {
            break;
        }
        current = (current + 1) % 2;
    }
    println!("{}", scores[(current + 1) % 2] * count);
}
