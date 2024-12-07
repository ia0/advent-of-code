use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::parse_ints;
use anyhow::Result;

fn concat(x: i64, y: i64) -> i64 {
    x * 10i64.pow(y.ilog10() + 1) + y
}

fn check(y: i64, xs: &[i64]) -> bool {
    let mut ops = vec![0; xs.len()];
    while ops[ops.len() - 1] == 0 {
        let mut z = xs[0];
        for (k, &x) in xs[1 ..].iter().enumerate() {
            match ops[k] {
                0 => z += x,
                1 => z *= x,
                2 => z = concat(z, x),
                _ => unreachable!(),
            }
        }
        if y == z {
            return true;
        }
        for op in &mut ops {
            *op += 1;
            if *op < 3 {
                break;
            }
            *op = 0;
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

adventofcode::main!(solve("examples/07.txt") == "249943041417600\n");
