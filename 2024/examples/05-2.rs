use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{Context, Result};

fn reorder(constraints: &HashSet<(i64, i64)>, xs: &mut [i64]) {
    for i in 0 .. xs.len() - 1 {
        'retry: loop {
            for j in i + 1 .. xs.len() {
                if constraints.contains(&(xs[j], xs[i])) {
                    xs.swap(i, j);
                    continue 'retry;
                }
            }
            break;
        }
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut constraints = HashSet::<(i64, i64)>::new();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (prev, next) = line.split_once('|').context("no pipe")?;
        assert!(constraints.insert((prev.parse()?, next.parse()?)));
    }
    let mut total = 0;
    'main: for line in lines {
        let mut xs =
            line?.split(',').map(|x| anyhow::Ok(x.parse()?)).collect::<Result<Vec<i64>>>()?;
        for j in 1 .. xs.len() {
            for i in 0 .. j {
                if constraints.contains(&(xs[j], xs[i])) {
                    reorder(&constraints, &mut xs);
                    total += xs[xs.len() / 2];
                    continue 'main;
                }
            }
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "5169\n");
