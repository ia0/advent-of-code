use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

fn parse_pos(mut xs: &[u8]) -> Coord {
    let mut pos = Coord::default();
    while !xs.is_empty() {
        match xs[0] {
            b'e' => pos.x += 1,
            b'w' => pos.x -= 1,
            b'n' => {
                pos.y += 1;
                if xs[1] == b'e' {
                    xs = &xs[1 ..];
                }
            }
            b's' => {
                pos.y -= 1;
                if xs[1] == b'w' {
                    xs = &xs[1 ..];
                }
            }
            _ => unreachable!(),
        }
        xs = &xs[1 ..];
    }
    pos
}

fn main() {
    let input = File::open("examples/24.txt").unwrap();
    let mut black = HashSet::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let pos = parse_pos(line.as_bytes());
        assert!(black.insert(pos) || black.remove(&pos));
    }
    println!("{}", black.len());
}
