use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = [i64; 3];

fn area(cubes: &HashSet<Coord>, x: Coord) -> usize {
    let mut r = 6;
    for d in 0 .. 3 {
        for s in [-1, 1] {
            let mut y = x;
            y[d] += s;
            r -= cubes.contains(&y) as usize;
        }
    }
    r
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut cubes = HashSet::<Coord>::new();
    for line in BufReader::new(input).lines() {
        let coord = line?.split(',').map(|x| x.parse()).collect::<Result<Vec<i64>, _>>()?;
        assert_eq!(coord.len(), 3);
        assert!(cubes.insert(Coord::try_from(coord).unwrap()));
    }
    writeln!(output, "{}", cubes.iter().map(|x| area(&cubes, *x)).sum::<usize>())?;
    Ok(())
}

adventofcode::main!(solve("examples/18.txt") == "4444\n");
