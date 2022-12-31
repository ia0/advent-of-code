use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn is_nice(xs: &[u8]) -> bool {
    xs.iter().filter(|x| b"aeiou".contains(x)).count() >= 3
        && xs.windows(2).any(|x| x[0] == x[1])
        && xs.windows(2).all(|x| ![b"ab", b"cd", b"pq", b"xy"].contains(&x.try_into().unwrap()))
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?.into_bytes();
        total += is_nice(&line) as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "255\n");
