use std::io::{Read, Write};

use anyhow::{Context, Result};
use regex::Regex;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut message = String::new();
    input.read_to_string(&mut message)?;
    let captures = Regex::new(
        "To continue, please consult the code grid in the manual\\.  Enter the code at row (.*), \
         column (.*)\\.",
    )?
    .captures(&message)
    .context("could not match message")?;
    let row = captures[1].parse::<usize>()? - 1;
    let column = captures[2].parse::<usize>()? - 1;
    let n = row + column;
    let count = n * (n + 1) / 2 + column;
    let mut code = 20151125u64;
    for _ in 0 .. count {
        code = (code * 252533u64) % 33554393u64;
    }
    writeln!(output, "{code}")?;
    Ok(())
}

adventofcode::main!(solve("examples/25.txt") == "2650453\n");
