#![feature(try_blocks)]

use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut list = BufReader::new(input)
        .lines()
        .map(|x| try { (x?.parse::<i64>()?, false) })
        .collect::<Result<Vec<_>>>()?;
    let n = list.len() as i64;
    let mut i = 0;
    while i < n {
        let (mut diff, ref mut done) = list[i as usize];
        if *done {
            i += 1;
            continue;
        }
        *done = true;
        match diff.cmp(&0) {
            Ordering::Less => diff -= -diff / n,
            Ordering::Equal => (),
            Ordering::Greater => diff += diff / n,
        }
        let j = (i + diff).rem_euclid(n);
        match (j.cmp(&i), diff > 0) {
            (Ordering::Less, false) => list[j as usize ..= i as usize].rotate_right(1),
            (Ordering::Less, true) => list[j as usize + 1 ..= i as usize].rotate_right(1),
            (Ordering::Equal, _) => (),
            (Ordering::Greater, false) => list[i as usize .. j as usize].rotate_left(1),
            (Ordering::Greater, true) => list[i as usize ..= j as usize].rotate_left(1),
        }
    }
    let list = list.into_iter().map(|(x, _)| x).collect::<Vec<_>>();
    let i = list.iter().position(|&x| x == 0).unwrap() as i64;
    writeln!(
        output,
        "{}",
        [1000, 2000, 3000].iter().map(|x| list[(i + x).rem_euclid(n) as usize]).sum::<i64>()
    )?;
    Ok(())
}

adventofcode::main!(solve("examples/20.txt") == "6640\n");
