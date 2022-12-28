use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/10.txt").unwrap();
    let mut input: Vec<i64> =
        BufReader::new(input).lines().map(|x| x.unwrap().parse().unwrap()).collect();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    let mut diffs = [0; 4];
    for i in 1 .. input.len() {
        diffs[(input[i] - input[i - 1]) as usize] += 1;
    }
    println!("{}", diffs[1] * diffs[3]);
}
