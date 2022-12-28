use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn adjacent(p: [i64; 3]) -> [[i64; 3]; 26] {
    [
        [p[0] - 1, p[1] - 1, p[2] - 1],
        [p[0] - 1, p[1] - 1, p[2]],
        [p[0] - 1, p[1] - 1, p[2] + 1],
        [p[0] - 1, p[1], p[2] - 1],
        [p[0] - 1, p[1], p[2]],
        [p[0] - 1, p[1], p[2] + 1],
        [p[0] - 1, p[1] + 1, p[2] - 1],
        [p[0] - 1, p[1] + 1, p[2]],
        [p[0] - 1, p[1] + 1, p[2] + 1],
        [p[0], p[1] - 1, p[2] - 1],
        [p[0], p[1] - 1, p[2]],
        [p[0], p[1] - 1, p[2] + 1],
        [p[0], p[1], p[2] - 1],
        [p[0], p[1], p[2] + 1],
        [p[0], p[1] + 1, p[2] - 1],
        [p[0], p[1] + 1, p[2]],
        [p[0], p[1] + 1, p[2] + 1],
        [p[0] + 1, p[1] - 1, p[2] - 1],
        [p[0] + 1, p[1] - 1, p[2]],
        [p[0] + 1, p[1] - 1, p[2] + 1],
        [p[0] + 1, p[1], p[2] - 1],
        [p[0] + 1, p[1], p[2]],
        [p[0] + 1, p[1], p[2] + 1],
        [p[0] + 1, p[1] + 1, p[2] - 1],
        [p[0] + 1, p[1] + 1, p[2]],
        [p[0] + 1, p[1] + 1, p[2] + 1],
    ]
}

fn step(old: &HashSet<[i64; 3]>) -> HashSet<[i64; 3]> {
    let mut count = HashMap::new();
    for &p in old {
        for &c in adjacent(p).iter() {
            *count.entry(c).or_default() += 1;
        }
    }
    let mut new = HashSet::new();
    for (&p, &c) in count.iter() {
        if (old.contains(&p) && 2 <= c && c <= 3) || (!old.contains(&p) && c == 3) {
            new.insert(p);
        }
    }
    new
}

fn main() {
    let input = File::open("examples/17.txt").unwrap();
    let mut state = HashSet::new();
    for (y, line) in BufReader::new(input).lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.bytes().enumerate() {
            match c {
                b'.' => (),
                b'#' => assert!(state.insert([x as i64, y as i64, 0])),
                _ => unreachable!(),
            }
        }
    }
    for _ in 0 .. 6 {
        state = step(&state);
    }
    println!("{}", state.len());
}
