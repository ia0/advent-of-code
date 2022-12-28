use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn count(xs: &[u8]) -> usize {
    let h: HashSet<u8> = xs.iter().cloned().collect();
    h.len()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let input = lines.next().unwrap()?.into_bytes();
    assert!(lines.next().is_none());
    for i in 3 .. input.len() {
        if count(&input[i - 3 ..= i]) == 4 {
            writeln!(output, "{}", i + 1)?;
            return Ok(());
        }
    }
    unreachable!();
}

adventofcode::main!(solve("examples/06.txt") == "1210\n");
