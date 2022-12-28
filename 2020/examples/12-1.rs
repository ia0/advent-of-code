use std::fs::File;
use std::io::{BufRead, BufReader};

use adventofcode::Coord;

fn main() {
    let input = File::open("examples/12.txt").unwrap();
    let mut pos = Coord { x: 0, y: 0 }; // x = E, y = N
    let mut dir = Coord { x: 1, y: 0 };
    for line in BufReader::new(input).lines() {
        let mut line = line.unwrap();
        let op = line.remove(0);
        let val: i64 = line.parse().unwrap();
        match op {
            'N' => pos += Coord { x: 0, y: 1 } * val,
            'S' => pos += Coord { x: 0, y: -1 } * val,
            'E' => pos += Coord { x: 1, y: 0 } * val,
            'W' => pos += Coord { x: -1, y: 0 } * val,
            'F' => pos += dir * val,
            'R' => {
                assert_eq!(val % 90, 0);
                for _ in 0 .. val / 90 {
                    dir = Coord { x: dir.y, y: -dir.x };
                }
            }
            'L' => {
                assert_eq!(val % 90, 0);
                for _ in 0 .. val / 90 {
                    dir = Coord { x: -dir.y, y: dir.x };
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", pos.x.abs() + pos.y.abs());
}
