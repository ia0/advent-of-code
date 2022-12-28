use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn solve(positions: [usize; 2], scores: [usize; 2], current: bool) -> [usize; 2] {
    let mut result = [0; 2];
    for (sum, count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut new_positions = positions;
        let mut new_scores = scores;
        let p = &mut new_positions[current as usize];
        *p = (*p + sum) % 10;
        if *p == 0 {
            *p = 10;
        }
        let s = &mut new_scores[current as usize];
        *s += *p;
        if *s >= 21 {
            result[current as usize] += count;
            continue;
        }
        for (r, x) in result.iter_mut().zip(solve(new_positions, new_scores, !current).iter()) {
            *r += count * x;
        }
    }
    result
}

fn main() {
    let input = File::open("examples/21.txt").unwrap();
    let regex = Regex::new(r#"starting position: ([0-9]+)"#).unwrap();
    let positions = BufReader::new(input)
        .lines()
        .map(|line| regex.captures(&line.unwrap()).unwrap()[1].parse().unwrap())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
    println!("{}", solve(positions, [0; 2], false).iter().max().unwrap());
}
