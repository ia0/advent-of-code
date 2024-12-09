use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut input = lines.next().unwrap()?.into_bytes();
    assert!(lines.next().is_none());
    assert!(input.iter().all(u8::is_ascii_digit));
    let mut block = Vec::with_capacity(input.len() + 1);
    block.push(0);
    for x in &mut input {
        *x -= b'0';
        block.push(block.last().unwrap() + *x as usize);
    }
    let mut total = 0;
    for last in (0 ..= (input.len() - 1) / 2).rev() {
        let mut pos = block[2 * last];
        for i in 0 .. last {
            if input[2 * last] <= input[2 * i + 1] {
                pos = block[2 * i + 1];
                block[2 * i + 1] += input[2 * last] as usize;
                input[2 * i + 1] -= input[2 * last];
                break;
            }
        }
        for i in 0 .. input[2 * last] {
            total += (pos + i as usize) * last;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "6307653242596\n");
