use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use regex::{Captures, Regex};

fn range(x: Captures) -> RangeInclusive<i64> {
    x[1].parse().unwrap() ..= x[2].parse().unwrap()
}

fn main() {
    let input = File::open("examples/17.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let input = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());
    let range_regex = Regex::new(r#"=(-?\d+)\.\.(-?\d+)"#).unwrap();
    let mut captures = range_regex.captures_iter(&input);
    let _xs = range(captures.next().unwrap());
    let ys = range(captures.next().unwrap());
    assert!(captures.next().is_none());
    assert!(*ys.start() < 0);
    let y = -ys.start() - 1;
    println!("{}", y * (y + 1) / 2);
}
