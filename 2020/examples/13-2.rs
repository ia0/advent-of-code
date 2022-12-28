use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve(x: i64, y: i64, t: i64) -> (i64, i64) {
    let mut b = None;
    for i in 0 .. {
        if (i * x).rem_euclid(t) == y {
            match b {
                None => b = Some(i),
                Some(b) => return (i - b, b),
            }
        }
    }
    unreachable!()
}

fn main() {
    let input = File::open("examples/13.txt").unwrap();
    let buses: String = BufReader::new(input).lines().nth(1).unwrap().unwrap();
    let buses: Vec<(i64, i64)> = buses
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| if x == "x" { None } else { Some((i as i64, x.parse().unwrap())) })
        .collect();
    let mut a: i64 = 1;
    let mut b = 0;
    for (i, t) in buses {
        let y = (-b - i).rem_euclid(t);
        let x = a.rem_euclid(t);
        let (c, d) = solve(x, y, t);
        b += a * d;
        a *= c;
        b %= a;
    }
    println!("{}", b);
}
