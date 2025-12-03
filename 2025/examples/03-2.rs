use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn max(mut bank: &[u8]) -> usize {
    let mut joltage = 0;
    const N: usize = 12;
    for i in 0 .. N {
        let mut j = 0;
        for k in 1 .. bank.len() - (N - i - 1) {
            if bank[j] < bank[k] {
                j = k;
            }
        }
        joltage = 10 * joltage + bank[j] as usize;
        bank = &bank[j + 1 ..];
    }
    joltage
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

adventofcode::main!(solve("examples/03.txt") == "171419245422055\n");
