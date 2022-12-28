extern crate number_encoding;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use number_encoding::greatest_common_divisor;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn dist(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn step(a: usize, b: usize, i: usize) -> usize {
    if a < b {
        a + i
    } else {
        a - i
    }
}

fn is_visible(space: &HashSet<Coord>, a: Coord, b: Coord) -> bool {
    let dx = dist(a.x, b.x);
    let dy = dist(a.y, b.y);
    let g = greatest_common_divisor(dx, dy);
    for i in 1 .. g {
        let x = step(a.x, b.x, i * dx / g);
        let y = step(a.y, b.y, i * dy / g);
        if space.contains(&Coord { x, y }) {
            return false;
        }
    }
    true
}

fn main() {
    let file = File::open("examples/10.txt").unwrap();
    let mut space = HashSet::new();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        for (x, &byte) in line.as_bytes().iter().enumerate() {
            if byte == b'#' {
                assert!(space.insert(Coord { x, y }));
            } else {
                assert_eq!(byte, b'.');
            }
        }
    }
    println!(
        "{}",
        space
            .iter()
            .map(|&a| { space.iter().filter(|&&b| { a != b && is_visible(&space, a, b) }).count() })
            .max()
            .unwrap()
    );
}
