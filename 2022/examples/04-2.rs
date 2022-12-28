use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{ensure, Context, Error, Result};

struct Interval {
    min: usize,
    max: usize,
}

impl FromStr for Interval {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut words = input.split('-');
        let min = words.next().context("no min")?.parse()?;
        let max = words.next().context("no max")?.parse()?;
        ensure!(words.next().is_none(), "extra input");
        ensure!(min <= max, "bad order");
        Ok(Interval { min, max })
    }
}

impl Interval {
    fn overlap(&self, other: &Interval) -> bool {
        self.min <= other.max && other.min <= self.max
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let mut words = line.split(',');
        let a: Interval = words.next().context("no a")?.parse()?;
        let b: Interval = words.next().context("no b")?.parse()?;
        ensure!(words.next().is_none(), "extra input");
        total += a.overlap(&b) as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "794\n");
