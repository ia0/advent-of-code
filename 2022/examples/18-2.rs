use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = [i64; 3];

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut cubes = HashSet::<Coord>::new();
    for line in BufReader::new(input).lines() {
        let coord = line?.split(',').map(|x| x.parse()).collect::<Result<Vec<i64>, _>>()?;
        assert_eq!(coord.len(), 3);
        assert!(cubes.insert(Coord::try_from(coord).unwrap()));
    }
    let (mut min, mut max) = ([i64::MAX; 3], [i64::MIN; 3]);
    for &cube in &cubes {
        for i in 0 .. 3 {
            min[i] = std::cmp::min(min[i], cube[i]);
            max[i] = std::cmp::max(max[i], cube[i]);
        }
    }
    for i in 0 .. 3 {
        min[i] -= 1;
        max[i] += 1;
    }
    let mut visited = HashSet::new();
    let mut todo = vec![min];
    let mut area = 0;
    while let Some(cur) = todo.pop() {
        if !visited.insert(cur) || (0 .. 3).any(|i| cur[i] < min[i] || max[i] < cur[i]) {
            continue;
        }
        for d in 0 .. 3 {
            for s in [-1, 1] {
                let mut next = cur;
                next[d] += s;
                if cubes.contains(&next) {
                    area += 1;
                } else {
                    todo.push(next);
                }
            }
        }
    }
    writeln!(output, "{area}")?;
    Ok(())
}

adventofcode::main!(solve("examples/18.txt") == "2530\n");
