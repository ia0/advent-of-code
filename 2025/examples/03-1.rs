use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn max(bank: &[u8]) -> usize {
    let mut i = 0;
    for j in 1 .. bank.len() - 1 {
        if bank[i] < bank[j] {
            i = j;
        }
    }
    let mut j = i + 1;
    for k in i + 2 .. bank.len() {
        if bank[j] < bank[k] {
            j = k;
        }
    }
    bank[i] as usize * 10 + bank[j] as usize
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let mut bank = line?.into_bytes();
        bank.iter_mut().for_each(|x| *x -= b'0');
        total += max(&bank);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "17311\n");
