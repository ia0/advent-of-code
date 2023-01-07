use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{bail, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut code = String::new();
    let mut pos = 5;
    for line in BufReader::new(input).lines() {
        for c in line?.chars() {
            match c {
                'U' if 3 < pos => pos -= 3,
                'D' if pos < 7 => pos += 3,
                'R' if pos % 3 != 0 => pos += 1,
                'L' if pos % 3 != 1 => pos -= 1,
                'U' | 'D' | 'R' | 'L' => (),
                _ => bail!("bad instruction {c}"),
            }
        }
        code.push((b'0' + pos) as char);
    }
    writeln!(output, "{code}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "33444\n");
