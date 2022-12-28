use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn main() {
    let input = File::open("examples/06.txt").unwrap();
    let mut count = 0;
    let mut yes: BTreeSet<u8> = (b'a' ..= b'z').collect();
    for line in BufReader::new(input).lines().chain(iter::once(Ok(String::new()))) {
        let line = line.unwrap().into_bytes();
        if line.is_empty() {
            count += yes.len();
            yes = (b'a' ..= b'z').collect();
            continue;
        }
        yes = yes.intersection(&line.into_iter().collect()).cloned().collect();
    }
    println!("{}", count);
}
