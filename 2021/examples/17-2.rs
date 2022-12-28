use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use regex::{Captures, Regex};

fn range(x: Captures) -> RangeInclusive<i64> {
    x[1].parse().unwrap() ..= x[2].parse().unwrap()
}

fn hits(txs: &RangeInclusive<i64>, tys: &RangeInclusive<i64>, mut vx: i64, mut vy: i64) -> bool {
    let (mut x, mut y) = (0, 0);
    while y >= *tys.start() && (vx > 0 || txs.contains(&x)) {
        if txs.contains(&x) && tys.contains(&y) {
            return true;
        }
        x += vx;
        y += vy;
        vx -= (vx > 0) as i64;
        vy -= 1;
    }
    false
}

fn main() {
    let input = File::open("examples/17.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let input = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());
    let range_regex = Regex::new(r#"=(-?\d+)\.\.(-?\d+)"#).unwrap();
    let mut captures = range_regex.captures_iter(&input);
    let txs = range(captures.next().unwrap());
    let tys = range(captures.next().unwrap());
    assert!(captures.next().is_none());
    let mut count = 0;
    for vx in 0 ..= *txs.end() {
        for vy in *tys.start() ..= (-tys.start() - 1) {
            count += hits(&txs, &tys, vx, vy) as usize;
        }
    }
    println!("{}", count);
}
