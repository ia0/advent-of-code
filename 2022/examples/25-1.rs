use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn value(snafu: &[u8]) -> i64 {
    let mut result = 0;
    for &x in snafu {
        let digit = match x {
            b'2' => 2,
            b'1' => 1,
            b'0' => 0,
            b'-' => -1,
            b'=' => -2,
            _ => unreachable!(),
        };
        result = 5 * result + digit;
    }
    result
}

fn snafu(mut value: i64) -> Vec<u8> {
    let mut result = vec![];
    let mut power = 1;
    let mut min = -2;
    let mut max = 2;
    while value < min || max < value {
        power *= 5;
        min = 5 * min - 2;
        max = 5 * max + 2;
    }
    while power > 0 {
        min = (min + 2) / 5;
        max = (max - 2) / 5;
        for i in -2 ..= 2 {
            let next = value - i * power;
            if min <= next && next <= max {
                result.push(b"=-012"[(2 + i) as usize]);
                value = next;
                break;
            }
        }
        power /= 5;
    }
    result
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        total += value(line?.as_bytes());
    }
    writeln!(output, "{}", std::str::from_utf8(&snafu(total))?)?;
    Ok(())
}

adventofcode::main!(solve("examples/25.txt") == "2=12-100--1012-0=012\n");
