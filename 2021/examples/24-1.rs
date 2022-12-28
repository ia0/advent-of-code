use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref R: Vec<Regex> = [
        "inp w",
        "mul x 0",
        "add x z",
        "mod x 26",
        "div z (.*)",
        "add x (.*)",
        "eql x w",
        "eql x 0",
        "mul y 0",
        "add y 25",
        "mul y x",
        "add y 1",
        "mul z y",
        "mul y 0",
        "add y w",
        "add y (.*)",
        "mul y x",
        "add z y",
    ]
    .iter()
    .map(|x| Regex::new(&format!("^{}$", x)).unwrap())
    .collect();
}

fn step(z: i64, v: i64, p: &[i64; 3]) -> i64 {
    let t = (z % 26 + p[1] != v) as i64;
    (z / p[0]) * (25 * t + 1) + (v + p[2]) * t
}

fn main() {
    let input = File::open("examples/24.txt").unwrap();
    let program: Vec<_> = BufReader::new(input).lines().map(|x| x.unwrap()).collect();
    let n = R.len();
    let mut ps: Vec<[i64; 3]> = Vec::new();
    for i in 0 .. 14 {
        let mut p: Vec<i64> = Vec::new();
        for j in 0 .. n {
            let c = R[j].captures(&program[i * n + j]).unwrap();
            match c.len() {
                1 => (),
                2 => p.push(c[1].parse().unwrap()),
                _ => unreachable!(),
            }
        }
        ps.push(p.try_into().unwrap());
    }
    const N: i64 = 10000000;
    let mut zs = HashMap::new();
    zs.insert(0, Vec::new());
    for i in 0 .. 14 {
        let mut nzs = HashMap::new();
        for (z, vs) in zs {
            for v in 1 ..= 9 {
                let nz = step(z, v, &ps[i]);
                if nz < N {
                    let mut nvs = vs.clone();
                    nvs.push(v);
                    let k = nzs.entry(nz).or_default();
                    if nvs > *k {
                        *k = nvs;
                    }
                }
            }
        }
        zs = nzs;
    }
    for &x in zs.get(&0).unwrap() {
        print!("{}", (b'0' + x as u8) as char);
    }
    println!();
}
