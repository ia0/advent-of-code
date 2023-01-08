use std::io::{Read, Write};

use anyhow::Result;

fn decode(mut encoded: &str) -> Result<usize> {
    let mut decoded = 0usize;
    while !encoded.is_empty() {
        let (prefix, rest) = match encoded.split_once('(') {
            Some(x) => x,
            None => {
                decoded += encoded.len();
                break;
            }
        };
        decoded += prefix.len();
        let (command, suffix) = rest.split_once(')').unwrap();
        let (length, count) = command.split_once('x').unwrap();
        let (data, suffix) = suffix.split_at(length.parse()?);
        encoded = suffix;
        let data_len = decode(data)?;
        decoded += count.parse::<usize>()? * data_len;
    }
    Ok(decoded)
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut encoded = String::new();
    input.read_to_string(&mut encoded)?;
    assert_eq!(encoded.pop(), Some('\n'));
    writeln!(output, "{}", decode(&encoded)?)?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "11317278863\n");
