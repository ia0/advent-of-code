use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/09.txt").unwrap();
    let goal = 393911906;
    let input: Vec<i64> =
        BufReader::new(input).lines().map(|x| x.unwrap().parse().unwrap()).collect();
    for i in 0 .. input.len() - 1 {
        let mut sum = input[i];
        for j in i + 1 .. input.len() {
            sum += input[j];
            if sum == goal {
                let min = input[i ..= j].iter().min().unwrap();
                let max = input[i ..= j].iter().max().unwrap();
                println!("{}", min + max);
                return;
            }
        }
    }
}
