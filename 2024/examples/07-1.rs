use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::parse_ints;
use anyhow::Result;

fn check(y: i64, xs: &[i64]) -> bool {
    for i in 0 .. 1 << (xs.len() - 1) {
        let mut z = xs[0];
        for (k, x) in xs[1 ..].iter().enumerate() {
            if i & 1 << k == 0 {
                z += x;
            } else {
                z *= x;
            }
        }
        if y == z {
            return true;
        }
    }
    false
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (y, xs) = line.split_once(": ").unwrap();
        let y: i64 = y.parse()?;
        let xs = parse_ints(xs)?;
        if check(y, &xs) {
            total += y;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/07.txt") == "2941973819040\n");
