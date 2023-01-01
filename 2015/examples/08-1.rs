use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut encoded = 0;
    let mut decoded = 0;
    for line in BufReader::new(input).lines() {
        let mut line = line?.into_bytes();
        encoded += line.len();
        assert_eq!(line[0], b'"');
        assert_eq!(line.pop(), Some(b'"'));
        let mut i = 1;
        while i < line.len() {
            decoded += 1;
            match line[i] {
                b'\\' => match line[i + 1] {
                    b'\\' | b'"' => i += 2,
                    b'x' => i += 4,
                    _ => unreachable!(),
                },
                _ => i += 1,
            }
        }
    }
    writeln!(output, "{}", encoded - decoded)?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "1342\n");
