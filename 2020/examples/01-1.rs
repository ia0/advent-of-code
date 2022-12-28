use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/01.txt").unwrap();
    let input: Vec<i64> =
        BufReader::new(input).lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let n = input.len();
    for i in 0 .. n - 1 {
        for j in i + 1 .. n {
            if input[i] + input[j] == 2020 {
                println!("{}", input[i] * input[j]);
                return;
            }
        }
    }
}
