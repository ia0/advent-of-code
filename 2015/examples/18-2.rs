use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::ADJACENT_STAR;
use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

fn step(cur: &HashSet<Coord>) -> HashSet<Coord> {
    let mut next = HashSet::new();
    for x in 1 ..= 100 {
        for y in 1 ..= 100 {
            let coord = Coord { x, y };
            if [1, 100].contains(&x) && [1, 100].contains(&y) {
                assert!(next.insert(coord));
                continue;
            }
            let count = ADJACENT_STAR.iter().filter(|&&step| cur.contains(&(coord + step))).count();
            if matches!((cur.contains(&coord), count), (true, 2) | (_, 3)) {
                assert!(next.insert(coord));
            }
        }
    }
    next
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut grid = HashSet::new();
    for (line, y) in BufReader::new(input).lines().zip(1 ..) {
        for (byte, x) in line?.bytes().zip(1 ..) {
            let coord = Coord { x, y };
            match byte {
                b'.' => (),
                b'#' => assert!(grid.insert(coord)),
                _ => unreachable!(),
            }
        }
    }
    for _ in 0 .. 100 {
        grid = step(&grid);
    }
    writeln!(output, "{}", grid.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/18.txt") == "781\n");
