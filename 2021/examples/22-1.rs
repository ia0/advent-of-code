use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

type Coord = [i64; 3];
type Cuboid = [RangeInclusive<i64>; 3];

struct Step {
    cuboid: Cuboid,
    on: bool,
}

impl Step {
    fn valid(&self) -> bool {
        self.cuboid.iter().all(|r| {
            let (a, b) = (*r.start(), *r.end());
            -50 <= a && a <= 50 && -50 <= b && b <= 50
        })
    }

    fn eval(&self, x: Coord) -> Option<bool> {
        if (0 .. 3).all(|k| self.cuboid[k].contains(&x[k])) {
            Some(self.on)
        } else {
            None
        }
    }
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"(-?[0-9]+)..(-?[0-9]+)"#).unwrap();
}

fn parse_range(c: Captures) -> RangeInclusive<i64> {
    c[1].parse().unwrap() ..= c[2].parse().unwrap()
}

fn parse(line: String) -> Step {
    let c: Vec<_> = REGEX.captures_iter(&line).map(parse_range).collect();
    Step { cuboid: c.try_into().unwrap(), on: line.as_bytes()[1] == b'n' }
}

fn main() {
    let input = File::open("examples/22.txt").unwrap();
    let mut steps: Vec<Step> =
        BufReader::new(input).lines().map(|x| parse(x.unwrap())).filter(Step::valid).collect();
    steps.reverse();
    let mut count = 0;
    let mut x = [-50; 3];
    'main: loop {
        count += steps.iter().find_map(|s| s.eval(x)).unwrap_or(false) as usize;
        for k in 0 .. 3 {
            x[k] += 1;
            if x[k] <= 50 {
                break;
            }
            if k == 2 {
                break 'main;
            }
            x[k] = -50;
        }
    }
    println!("{}", count);
}
