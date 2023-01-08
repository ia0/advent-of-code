use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn has_abba(xs: &str) -> bool {
    xs.as_bytes().windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn is_tls(mut xs: &str) -> bool {
    let mut seen = false;
    while !xs.is_empty() {
        let (inside, outside, rest) = match xs.split_once('[') {
            Some((inside, rest)) => {
                let (outside, rest) = rest.split_once(']').unwrap();
                (inside, outside, rest)
            }
            None => (xs, "", ""),
        };
        if has_abba(outside) {
            return false;
        }
        seen = seen || has_abba(inside);
        xs = rest;
    }
    seen
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut count = 0;
    for line in BufReader::new(input).lines() {
        count += is_tls(&line?) as usize;
    }
    writeln!(output, "{count}")?;
    Ok(())
}

adventofcode::main!(solve("examples/07.txt") == "110\n");
