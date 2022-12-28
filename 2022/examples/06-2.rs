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
    let k = 14;
    for i in k ..= input.len() {
        if count(&input[i - k .. i]) == k {
            writeln!(output, "{i}")?;
            return Ok(());
        }
    }
    unreachable!();
}

adventofcode::main!(solve("examples/06.txt") == "3476\n");
