extern crate number_encoding;
extern crate regex;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use number_encoding::greatest_common_divisor;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    pos: i64,
    vel: i64,
}

fn gravity(this: i64, other: i64) -> i64 {
    match this.cmp(&other) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / greatest_common_divisor(x, y)
}

fn main() {
    let file = File::open("examples/12.txt").unwrap();
    let coord_regex = Regex::new("<x=(.*), y=(.*), z=(.*)>").unwrap();
    let mut moons = vec![Vec::new(); 3];
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let c = coord_regex.captures(&line).unwrap();
        assert_eq!(c.len(), 4);
        for i in 0 .. 3 {
            moons[i].push(Moon { pos: c[i + 1].parse().unwrap(), vel: 0 });
        }
    }
    let period: Vec<usize> = moons
        .into_iter()
        .map(|mut moons| {
            let initial_moons = moons.clone();
            let mut period = 0;
            loop {
                period += 1;
                let mut new_moons = moons.clone();
                for i in 0 .. moons.len() {
                    for j in 0 .. moons.len() {
                        if i == j {
                            continue;
                        }
                        new_moons[i].vel += gravity(moons[i].pos, moons[j].pos);
                    }
                    new_moons[i].pos += new_moons[i].vel;
                }
                moons = new_moons;
                if moons == initial_moons {
                    return period;
                }
            }
        })
        .collect();
    println!("{}", lcm(lcm(period[0], period[1]), period[2]));
}
