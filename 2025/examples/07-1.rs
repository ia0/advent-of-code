use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut beams = HashSet::new();
    let mut lines = BufReader::new(input).lines();
    beams.insert(lines.next().unwrap()?.bytes().position(|x| x == b'S').unwrap());
    let mut total = 0;
    for line in lines {
        let mut next = HashSet::new();
        for (i, b) in line?.bytes().enumerate() {
            if beams.remove(&i) {
                if b == b'^' {
                    next.insert(i - 1);
                    next.insert(i + 1);
                    total += 1;
                } else {
                    next.insert(i);
                }
            }
        }
        beams = next;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/07.txt") == "1600\n");
