use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::order_pair_mut;
use anyhow::{ensure, Context, Result};
use rand::Rng;

fn parse(input: &str) -> Result<u16> {
    let xs = input.as_bytes();
    ensure!(input.len() == 3);
    ensure!(xs.iter().all(|x| x.is_ascii_lowercase()));
    let mut r = 0;
    for x in xs {
        r = 26 * r + (x - b'a') as u16;
    }
    Ok(r)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut edges = HashMap::<u16, HashSet<u16>>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (left, right) = line.split_once(": ").context("no :")?;
        let left = parse(left)?;
        for right in right.split_whitespace() {
            let right = parse(right)?;
            assert!(edges.entry(left).or_default().insert(right));
            assert!(edges.entry(right).or_default().insert(left));
        }
    }
    loop {
        let mut n = 0i64;
        let mut e = HashMap::new();
        let mut v = HashMap::new();
        for (&a, bs) in &edges {
            assert!(v.insert(a, 1).is_none());
            for &b in bs {
                assert!(a != b);
                if a < b {
                    assert!(e.insert((a, b), 1).is_none());
                    n += 1;
                }
            }
        }
        while 2 < v.len() {
            assert_eq!(e.values().sum::<i64>(), n);
            let mut r = rand::thread_rng().gen_range(0 .. n);
            let (a, b) = {
                let mut i = e.iter();
                loop {
                    let (&e, &c) = i.next().unwrap();
                    r -= c;
                    if r < 0 {
                        break e;
                    }
                }
            };
            *v.get_mut(&a).unwrap() += v.remove(&b).unwrap();
            n -= e.remove(&(a, b)).unwrap();
            let mut u = HashMap::<_, i64>::new();
            e.retain(|&e, &mut c| {
                let mut f = e;
                if f.0 == b {
                    f.0 = a;
                } else if f.1 == b {
                    f.1 = a;
                } else {
                    return true;
                }
                order_pair_mut(&mut f.0, &mut f.1);
                *u.entry(f).or_default() += c;
                false
            });
            for (u, c) in u {
                *e.entry(u).or_default() += c;
            }
        }
        assert_eq!(v.len(), 2);
        assert_eq!(e.len(), 1);
        if e.into_values().next().unwrap() == 3 {
            writeln!(output, "{}", v.into_values().product::<i64>())?;
            return Ok(());
        }
    }
}

adventofcode::main!(solve("examples/25.txt") == "606062\n");
