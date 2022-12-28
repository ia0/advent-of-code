use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Default)]
struct Mask {
    ones: u64,
    floats: Vec<usize>,
}

impl Mask {
    fn new(x: &[u8]) -> Mask {
        assert_eq!(x.len(), 36);
        let mut ones = 0;
        let mut floats = Vec::new();
        for (i, x) in x.iter().rev().enumerate() {
            match x {
                b'0' => (),
                b'1' => ones |= 1 << i,
                b'X' => floats.push(i),
                _ => unreachable!(),
            }
        }
        Mask { ones, floats }
    }

    fn apply(&self, mut x: u64) -> Vec<u64> {
        let mut r = Vec::new();
        x |= self.ones;
        let n = self.floats.len();
        for i in 0 .. 1 << n {
            for b in 0 .. n {
                if i & 1 << b == 0 {
                    x &= !(1 << self.floats[b]);
                } else {
                    x |= 1 << self.floats[b];
                }
            }
            r.push(x);
        }
        r
    }
}

fn main() {
    let input = File::open("examples/14.txt").unwrap();
    let mut mask = Mask::default();
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let regex = Regex::new(r#"^mem\[([0-9]+)\] = ([0-9]+)$"#).unwrap();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        if let Some(captures) = regex.captures(&line) {
            let k: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
            let v: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
            for k in mask.apply(k) {
                mem.insert(k, v);
            }
        } else {
            mask = Mask::new(line.strip_prefix("mask = ").unwrap().as_bytes());
        }
    }
    println!("{}", mem.values().sum::<u64>());
}
