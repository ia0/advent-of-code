use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Default)]
struct Mask {
    zeros: u64,
    ones: u64,
}

impl Mask {
    fn new(x: &[u8]) -> Mask {
        let mut zeros = 0;
        let mut ones = 0;
        assert_eq!(x.len(), 36);
        for (i, x) in x.iter().rev().enumerate() {
            match x {
                b'0' => zeros |= 1 << i,
                b'1' => ones |= 1 << i,
                b'X' => (),
                _ => unreachable!(),
            }
        }
        assert_eq!(zeros & ones, 0, "{:x}", zeros & ones);
        Mask { zeros, ones }
    }

    fn apply(&self, x: u64) -> u64 {
        (x | self.ones) & !self.zeros
    }
}

fn main() {
    let input = File::open("examples/14.txt").unwrap();
    let mut mask = Mask::default();
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let regex = Regex::new(r#"^mem\[([0-9]+)\] = ([0-9]+)$"#).unwrap();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        if let Some(captures) = regex.captures(&line) {
            let k: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let v: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
            mem.insert(k, mask.apply(v));
        } else {
            mask = Mask::new(line.strip_prefix("mask = ").unwrap().as_bytes());
        }
    }
    println!("{}", mem.values().sum::<u64>());
}
