#![feature(try_blocks)]

use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut list = BufReader::new(input)
        .lines()
        .enumerate()
        .map(|(i, x)| try { (x?.parse::<i64>()? * 811589153, i) })
        .collect::<Result<Vec<_>>>()?;
    let n = list.len();
    for _ in 0 .. 10 {
        for i in 0 .. n {
            let i = list.iter().position(|&(_, x)| x == i).unwrap();
            let mut diff = list[i].0;
            match diff.cmp(&0) {
                Ordering::Less => diff -= -diff / (n - 1) as i64,
                Ordering::Equal => (),
                Ordering::Greater => diff += diff / (n - 1) as i64,
            }
            let j = (i as i64 + diff).rem_euclid(n as i64) as usize;
            match (j.cmp(&i), diff > 0) {
                (Ordering::Less, false) => list[j ..= i].rotate_right(1),
                (Ordering::Less, true) => list[j + 1 ..= i].rotate_right(1),
                (Ordering::Equal, _) => (),
                (Ordering::Greater, false) => list[i .. j].rotate_left(1),
                (Ordering::Greater, true) => list[i ..= j].rotate_left(1),
            }
        }
    }
    let list = list.into_iter().map(|(x, _)| x).collect::<Vec<_>>();
    let i = list.iter().position(|&x| x == 0).unwrap();
    writeln!(
        output,
        "{:?}",
        [1000, 2000, 3000].iter().map(|x| list[(i + x).rem_euclid(n)]).sum::<i64>()
    )?;
    Ok(())
}

adventofcode::main!(solve("examples/20.txt") == "11893839037215\n");
