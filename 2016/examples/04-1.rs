use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut sum = 0;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let parts = line.split('-').collect::<Vec<_>>();
        let last = parts.len() - 1;
        let mut count = HashMap::<u8, usize>::new();
        for part in &parts[.. last] {
            for byte in part.bytes() {
                *count.entry(byte).or_default() += 1;
            }
        }
        let mut count = count.into_iter().map(|(b, c)| (Reverse(c), b)).collect::<Vec<_>>();
        count.sort();
        let act = count.into_iter().map(|(_, x)| x).take(5).collect::<Vec<_>>();
        let (id, exp) = parts[last].strip_suffix(']').unwrap().split_once('[').unwrap();
        if act == exp.as_bytes() {
            sum += id.parse::<usize>()?;
        }
    }
    writeln!(output, "{sum}")?;
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "137896\n");
