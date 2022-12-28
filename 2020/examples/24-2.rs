use std::collections::{HashMap, HashSet};
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

const ADJACENT: [Coord; 6] = [
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 1 },
    Coord { x: -1, y: 0 },
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: -1 },
];

fn main() {
    let input = File::open("examples/24.txt").unwrap();
    let mut black = HashSet::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let pos = parse_pos(line.as_bytes());
        assert!(black.insert(pos) || black.remove(&pos));
    }
    for _ in 0 .. 100 {
        let mut count: HashMap<_, usize> = HashMap::new();
        for &pos in &black {
            for &diff in ADJACENT.iter() {
                *count.entry(pos + diff).or_default() += 1;
            }
        }
        black = count
            .into_iter()
            .filter(|&(pos, count)| {
                (black.contains(&pos) && 1 <= count && count <= 2)
                    || (!black.contains(&pos) && count == 2)
            })
            .map(|(pos, _)| pos)
            .collect();
    }
    println!("{}", black.len());
}
