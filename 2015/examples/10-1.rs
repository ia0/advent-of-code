use std::io::{Read, Write};

use anyhow::Result;

fn step(xs: &[u8]) -> Vec<u8> {
    let mut ys = Vec::new();
    for xs in xs.chunk_by(|x, y| x == y) {
        assert!(xs.len() < 10);
        ys.push(xs.len() as u8 + b'0');
        ys.push(xs[0]);
    }
    ys
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut xs = Vec::new();
    input.read_to_end(&mut xs)?;
    for _ in 0 .. 40 {
        xs = step(&xs);
    }
    writeln!(output, "{}", xs.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/10.txt") == "492982\n");
