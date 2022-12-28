extern crate number_encoding;

use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use number_encoding::greatest_common_divisor;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

fn dist(a: i64, b: i64) -> usize {
    if a > b {
        (a - b) as usize
    } else {
        (b - a) as usize
    }
}

fn step(a: i64, b: i64, i: usize) -> i64 {
    if a < b {
        a + i as i64
    } else {
        a - i as i64
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
                let x = x as i64;
                let y = y as i64;
                assert!(space.insert(Coord { x, y }));
            } else {
                assert_eq!(byte, b'.');
            }
        }
    }
    let center = space
        .iter()
        .map(|&a| (a, space.iter().filter(|&&b| a != b && is_visible(&space, a, b)).count()))
        .max_by_key(|&(_, x)| x)
        .unwrap()
        .0;
    let mut space: Vec<Coord> = space
        .into_iter()
        .filter_map(|mut c| {
            c.x -= center.x;
            c.y -= center.y;
            if c.x == 0 && c.y == 0 {
                None
            } else {
                Some(c)
            }
        })
        .collect();
    space.sort_by(|a, b| {
        if (a.x >= 0) != (b.x >= 0) {
            b.x.cmp(&a.x)
        } else {
            (a.y * b.x)
                .cmp(&(b.y * a.x))
                .then(a.x.abs().cmp(&b.x.abs()).then(a.y.abs().cmp(&b.y.abs())))
        }
    });
    let mut space = VecDeque::from(space);
    let mut last = space.pop_front().unwrap();
    let mut i = 1;
    while let Some(cur) = space.pop_front() {
        if last.x * cur.y == last.y * cur.x {
            space.push_back(cur);
        } else {
            i += 1;
            if i == 200 {
                println!("{}", (center.x + cur.x) * 100 + (center.y + cur.y));
                return;
            }
            last = cur;
        }
    }
}
