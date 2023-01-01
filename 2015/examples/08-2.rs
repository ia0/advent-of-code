use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut encoded = 0;
    let mut decoded = 0;
    for line in BufReader::new(input).lines() {
        let line = line?.into_bytes();
        decoded += line.len();
        encoded += 2 + line
            .iter()
            .map(|x| match x {
                b'\\' => 2,
                b'"' => 2,
                _ => 1,
            })
            .sum::<usize>();
    }
    writeln!(output, "{}", encoded - decoded)?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "2074\n");
