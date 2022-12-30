use std::io::{Read, Write};

use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut directions = Vec::new();
    input.read_to_end(&mut directions)?;
    let mut floor = 0i64;
    for (direction, position) in directions.into_iter().zip(1 ..) {
        match direction {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor == -1 {
            writeln!(output, "{position}")?;
            break;
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "1771\n");
