use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    for line in BufReader::new(input).lines() {
        let line = line?;
        let mut parts = line.split('-').collect::<Vec<_>>();
        let (id, _) = parts.pop().unwrap().split_once('[').unwrap();
        let id = id.parse::<usize>()?;
        let mut name = String::new();
        for part in parts {
            for byte in part.bytes() {
                name.push(((((byte - b'a') as usize + id) % 26) as u8 + b'a') as char);
            }
            name.push(' ');
        }
        if name == "northpole object storage " {
            writeln!(output, "{id}")?;
            break;
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "501\n");
