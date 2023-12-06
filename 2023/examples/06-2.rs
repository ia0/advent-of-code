use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Context, Result};

fn numbers(input: &str) -> Result<i64> {
    Ok(input.replace(' ', "").parse()?)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let time = lines.next().context("no Time line")??;
    let dist = lines.next().context("no Distance line")??;
    ensure!(lines.next().is_none(), "too many lines");
    let time = numbers(time.strip_prefix("Time:").context("no Time prefix")?)?;
    let dist = numbers(dist.strip_prefix("Distance:").context("no Distance prefix")?)?;
    let delta = time * time - 4 * (dist + 1);
    ensure!(0 < delta, "delta is not positive");
    let sqrt = (delta as f64).sqrt();
    let low = ((time as f64) - sqrt) / 2.;
    let high = ((time as f64) + sqrt) / 2.;
    let result = high.floor() as i64 - low.ceil() as i64 + 1;
    writeln!(output, "{result}")?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "27340847\n");
