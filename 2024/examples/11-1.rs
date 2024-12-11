use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::parse_ints;
use anyhow::Result;

fn blink(xs: &mut Vec<i64>) {
    let mut ys = Vec::new();
    for &x in xs.iter() {
        if x == 0 {
            ys.push(1);
        } else if x.ilog10() % 2 == 1 {
            let m = 10i64.pow(x.ilog10() / 2 + 1);
            ys.push(x / m);
            ys.push(x % m);
        } else {
            ys.push(x * 2024);
        }
    }
    *xs = ys;
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut xs = parse_ints(&lines.next().unwrap()?)?;
    assert!(lines.next().is_none());
    for _ in 0 .. 25 {
        blink(&mut xs);
    }
    writeln!(output, "{}", xs.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "207683\n");
