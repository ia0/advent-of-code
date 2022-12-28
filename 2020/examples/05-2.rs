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
    ids.sort();
    let holes: Vec<_> = ids.windows(2).filter(|w| w[1] - w[0] > 1).collect();
    assert_eq!(holes.len(), 1);
    let hole = holes[0];
    assert_eq!(hole[1] - hole[0], 2);
    println!("{}", hole[0] + 1);
}
