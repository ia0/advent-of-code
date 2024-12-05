use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{Context, Result};

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
        let xs = line?.split(',').map(|x| anyhow::Ok(x.parse()?)).collect::<Result<Vec<i64>>>()?;
        for j in 1 .. xs.len() {
            for i in 0 .. j {
                if constraints.contains(&(xs[j], xs[i])) {
                    continue 'main;
                }
            }
        }
        total += xs[xs.len() / 2];
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "6242\n");
