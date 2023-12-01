use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?.into_bytes();
        let first = line.iter().find(|x| x.is_ascii_digit()).unwrap() - b'0';
        let last = line.iter().rfind(|x| x.is_ascii_digit()).unwrap() - b'0';
        total += (first * 10 + last) as u64;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "54304\n");
