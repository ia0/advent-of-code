use std::fs::File;
use std::io::{BufRead, BufReader};

use number_encoding::combinadics::Iter;
use number_encoding::combination;

fn main() {
    let input = File::open("examples/01.txt").unwrap();
    let input: Vec<i64> =
        BufReader::new(input).lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let n = input.len();
    let k = 3;
    let mut iter = Iter::new(k);
    for _ in 0 .. combination(n, k) {
        let xs = iter.get().iter().map(|&i| input[i]);
        if xs.clone().sum::<i64>() == 2020 {
            println!("{}", xs.clone().product::<i64>());
            return;
        }
        iter.advance();
    }
}
