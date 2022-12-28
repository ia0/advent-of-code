use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use adventofcode::{Coord, ADJACENT_STAR};

fn main() {
    let input = File::open("examples/11.txt").unwrap();
    let mut occupied = HashMap::new();
    for (y, line) in BufReader::new(input).lines().enumerate() {
        for (x, byte) in line.unwrap().bytes().enumerate() {
            let coord = Coord { x: x as i64, y: y as i64 };
            let is_occupied = match byte {
                b'.' => continue,
                b'L' => false,
                b'#' => true,
                _ => unreachable!(),
            };
            assert!(occupied.insert(coord, is_occupied).is_none());
        }
    }
    loop {
        let prev = occupied.clone();
        for (coord, occupied) in occupied.iter_mut() {
            let count = ADJACENT_STAR
                .iter()
                .map(|x| *coord + *x)
                .filter_map(|x| prev.get(&x))
                .filter(|&&x| x)
                .count();
            if *occupied && count >= 4 {
                *occupied = false;
            } else if !*occupied && count == 0 {
                *occupied = true;
            }
        }
        if occupied == prev {
            break;
        }
    }
    println!("{}", occupied.values().filter(|&&x| x).count());
}
