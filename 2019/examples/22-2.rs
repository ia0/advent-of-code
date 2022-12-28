use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Technique {
    Reverse,
    Increment(i128),
    Cut(i128),
}

impl FromStr for Technique {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Technique, Box<dyn Error>> {
        if input == "deal into new stack" {
            return Ok(Technique::Reverse);
        }
        if let Some(value) = input.strip_prefix("deal with increment ") {
            return Ok(Technique::Increment(value.parse()?));
        }
        if let Some(value) = input.strip_prefix("cut ") {
            return Ok(Technique::Cut(value.parse()?));
        }
        panic!("{}", input)
    }
}

fn extended_euclid(a: i128, b: i128) -> i128 {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    while r > 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    assert_eq!(old_r, 1);
    old_s
}

struct Affine {
    mul: i128,
    add: i128,
}

impl Technique {
    fn reverse(self, f: Affine, n: i128) -> Affine {
        match self {
            Technique::Reverse => {
                Affine { mul: (-f.mul).rem_euclid(n), add: (-f.add - 1).rem_euclid(n) }
            }
            Technique::Increment(v) => {
                assert_eq!(v, v % n);
                let r = extended_euclid(v, n) % n;
                Affine { mul: (f.mul * r) % n, add: (f.add * r) % n }
            }
            Technique::Cut(v) => Affine { mul: f.mul, add: (f.add + v) % n },
        }
    }
}

fn exp_mod(b: i128, mut e: i128, n: i128) -> i128 {
    let mut b = b.rem_euclid(n);
    let mut r = 1;
    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % n;
        }
        e /= 2;
        b = (b * b) % n;
    }
    r
}

fn main() {
    let input = File::open("examples/22.txt").unwrap();
    let shuffle: Vec<Technique> =
        BufReader::new(input).lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let n = 119315717514047;
    let mut f = Affine { mul: 1, add: 0 };
    for technique in shuffle.iter().rev() {
        f = technique.reverse(f, n);
    }
    let t = 101741582076661;
    let mul = exp_mod(f.mul, t, n);
    let rev = extended_euclid((f.mul - 1).rem_euclid(n), n) % n;
    let add = f.add * (mul - 1) % n * rev % n;
    let x = 2020;
    println!("{}", (mul * x + add) % n);
}
