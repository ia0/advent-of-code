use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    let mut pos = 50i32;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let mut delta = if let Some(delta) = line.strip_prefix("R") {
            delta.parse::<i32>().unwrap()
        } else if let Some(delta) = line.strip_prefix("L") {
            -delta.parse::<i32>().unwrap()
        } else {
            unreachable!()
        };
        while delta != 0 {
            if delta < 0 {
                pos -= 1;
                delta += 1;
                pos = pos.rem_euclid(100);
                total += (pos == 0) as i32;
            } else {
                pos += 1;
                delta -= 1;
                pos = pos.rem_euclid(100);
                total += (pos == 0) as i32;
            }
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "6498\n");
