use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn main() {
    let input = File::open("examples/06.txt").unwrap();
    let mut count = 0;
    let mut yes = HashSet::new();
    for line in BufReader::new(input).lines().chain(iter::once(Ok(String::new()))) {
        let line = line.unwrap().into_bytes();
        if line.is_empty() {
            count += yes.len();
            yes.clear();
        }
        for q in line {
            yes.insert(q);
        }
    }
    println!("{}", count);
}
