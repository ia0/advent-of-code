use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Context, Result};

fn hash(xs: &[u8]) -> u8 {
    let mut r = 0u8;
    for &x in xs {
        r = r.wrapping_add(x).wrapping_mul(17);
    }
    r
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let line = lines.next().context("empty input")??;
    ensure!(lines.next().is_none(), "extra input");
    let steps = line.split(',').collect::<Vec<_>>();
    let mut total = 0;
    for step in steps {
        total += hash(step.as_bytes()) as i64;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/15.txt") == "510273\n");
