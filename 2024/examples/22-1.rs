use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn mix_prune(secret: usize, new: usize) -> usize {
    (secret ^ new) % 16777216
}

fn next(mut secret: usize, count: usize) -> usize {
    for _ in 0 .. count {
        secret = mix_prune(secret, secret * 64);
        secret = mix_prune(secret, secret / 32);
        secret = mix_prune(secret, secret * 2048);
    }
    secret
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let secret = line?.parse()?;
        total += next(secret, 2000);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "17612566393\n");
