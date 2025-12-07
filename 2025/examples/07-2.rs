use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut beams = HashMap::new();
    let mut lines = BufReader::new(input).lines();
    beams.insert(lines.next().unwrap()?.bytes().position(|x| x == b'S').unwrap(), 1usize);
    for line in lines {
        let mut next = HashMap::new();
        for (i, b) in line?.bytes().enumerate() {
            if let Some(c) = beams.remove(&i) {
                if b == b'^' {
                    *next.entry(i - 1).or_default() += c;
                    *next.entry(i + 1).or_default() += c;
                } else {
                    *next.entry(i).or_default() += c;
                }
            }
        }
        beams = next;
    }
    writeln!(output, "{}", beams.into_values().sum::<usize>())?;
    Ok(())
}

adventofcode::main!(solve("examples/07.txt") == "8632253783011\n");
