use std::io::{Read, Write};

use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut encoded = String::new();
    input.read_to_string(&mut encoded)?;
    assert_eq!(encoded.pop(), Some('\n'));
    let mut encoded = encoded.as_str();
    let mut decoded = String::new();
    while !encoded.is_empty() {
        let (prefix, rest) = match encoded.split_once('(') {
            Some(x) => x,
            None => {
                decoded.push_str(encoded);
                break;
            }
        };
        decoded.push_str(prefix);
        let (command, suffix) = rest.split_once(')').unwrap();
        let (length, count) = command.split_once('x').unwrap();
        let (data, suffix) = suffix.split_at(length.parse()?);
        encoded = suffix;
        for _ in 0 .. count.parse::<usize>()? {
            decoded.push_str(data);
        }
    }
    writeln!(output, "{}", decoded.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "183269\n");
