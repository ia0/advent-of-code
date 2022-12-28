use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/05.txt").unwrap();
    let mut ids = Vec::new();
    for line in BufReader::new(input).lines() {
        let mut line = line.unwrap().into_bytes();
        line.reverse();
        let mut id = 0;
        while let Some(x) = line.pop() {
            id *= 2;
            match x {
                b'B' | b'R' => id += 1,
                b'F' | b'L' => (),
                _ => panic!(),
            }
        }
        ids.push(id);
    }
    println!("{}", ids.iter().max().unwrap());
}
