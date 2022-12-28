use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/01.txt").unwrap();
    let input: Vec<i64> =
        BufReader::new(input).lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let result = input.windows(2).filter(|x| x[0] < x[1]).count();
    println!("{}", result);
}
