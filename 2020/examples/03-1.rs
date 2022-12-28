use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/03.txt").unwrap();
    let mut count = 0;
    let mut x = 0;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap().into_bytes();
        count += (line[x] == b'#') as usize;
        x += 3;
        x %= line.len();
    }
    println!("{}", count);
}
