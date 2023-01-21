use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn get(xs: &[u8], i: i64) -> u8 {
    if (0 .. xs.len() as i64).contains(&i) {
        xs[i as usize]
    } else {
        b'.'
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut prev = lines.next().unwrap()?.into_bytes();
    assert!(lines.next().is_none());
    let mut total = prev.iter().filter(|&&x| x == b'.').count();
    for _ in 1 .. 400000 {
        let mut next = Vec::with_capacity(prev.len());
        for x in 0 .. prev.len() as i64 {
            next.push(if get(&prev, x - 1) == get(&prev, x + 1) { b'.' } else { b'^' });
        }
        prev = next;
        total += prev.iter().filter(|&&x| x == b'.').count();
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/18.txt") == "19998750\n");
