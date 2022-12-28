use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

type Coord = [i64; 3];
type Rotation = [[i64; 3]; 3];

lazy_static! {
    static ref ROTATIONS: Vec<Rotation> = {
        let range = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut r = HashSet::new();
        for a in range {
            for b in range {
                for c in range {
                    r.insert([
                        [a.0 * b.0, a.0 * b.1 * c.1 - a.1 * c.0, a.0 * b.1 * c.0 + a.1 * c.1],
                        [a.1 * b.0, a.1 * b.1 * c.1 + a.0 * c.0, a.1 * b.1 * c.0 - a.0 * c.1],
                        [-b.1, b.0 * c.1, b.0 * c.0],
                    ]);
                }
            }
        }
        r.into_iter().collect()
    };
}

fn rotate(r: &Rotation, x: Coord) -> Coord {
    let mut y = [0; 3];
    for i in 0 .. 3 {
        for j in 0 .. 3 {
            y[i] += r[i][j] * x[j];
        }
    }
    y
}

fn translate(t: Coord, x: Coord) -> Coord {
    let mut y = [0; 3];
    for k in 0 .. 3 {
        y[k] += t[k] + x[k];
    }
    y
}

fn dist(a: Coord, b: Coord) -> usize {
    let mut r = 0;
    for k in 0 .. 3 {
        r += (a[k] - b[k]).abs() as usize;
    }
    r
}

fn parse(input: &str) -> Coord {
    let v: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    assert_eq!(v.len(), 3);
    [v[0], v[1], v[2]]
}

fn merge(
    a: &mut (HashSet<Coord>, Vec<Coord>), b: &(HashSet<Coord>, Vec<Coord>), r: &Rotation,
) -> bool {
    let mut count: HashMap<Coord, usize> = HashMap::new();
    for &b in &b.0 {
        let b = rotate(r, b);
        for &a in a.0.iter() {
            let mut t = [0; 3];
            for k in 0 .. 3 {
                t[k] = a[k] - b[k];
            }
            *count.entry(t).or_default() += 1;
        }
    }
    count.retain(|_, c| *c >= 12);
    let mut count = count.into_iter();
    let t = match count.next() {
        None => return false,
        Some((x, _)) => x,
    };
    assert!(count.next().is_none());
    for &b in &b.0 {
        a.0.insert(translate(t, rotate(r, b)));
    }
    for &b in &b.1 {
        a.1.push(translate(t, rotate(r, b)));
    }
    true
}

fn main() {
    let input = File::open("examples/19.txt").unwrap();
    let scanner_regex = Regex::new(r#"^--- scanner ([0-9]+) ---$"#).unwrap();
    let mut scanners = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if let Some(capture) = scanner_regex.captures(&line) {
            assert_eq!(capture[1].parse::<usize>().unwrap(), scanners.len());
            scanners.push((HashSet::new(), vec![[0; 3]]));
        } else {
            assert!(scanners.last_mut().unwrap().0.insert(parse(&line)));
        }
    }
    'main: while scanners.len() > 1 {
        let n = scanners.len();
        for i in 1 .. n {
            let (sl, sr) = scanners.split_at_mut(i);
            for j in 0 .. i {
                for r in &*ROTATIONS {
                    if merge(&mut sl[j], &sr[0], r) {
                        scanners.swap_remove(i);
                        continue 'main;
                    }
                }
            }
        }
    }
    let mut max = 0;
    for i in 1 .. scanners[0].1.len() {
        for j in 0 .. i {
            max = std::cmp::max(max, dist(scanners[0].1[i], scanners[0].1[j]));
        }
    }
    println!("{}", max);
}
