use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn push_abas(xs: &str, rev: bool, set: &mut HashSet<(u8, u8)>) {
    for w in xs.as_bytes().windows(3) {
        if w[0] == w[2] && w[0] != w[1] {
            set.insert(if rev { (w[1], w[0]) } else { (w[0], w[1]) });
        }
    }
}

fn is_ssl(mut xs: &str) -> bool {
    let mut inside_abas = HashSet::new();
    let mut outside_abas = HashSet::new();
    while !xs.is_empty() {
        let (inside, outside, rest) = match xs.split_once('[') {
            Some((inside, rest)) => {
                let (outside, rest) = rest.split_once(']').unwrap();
                (inside, outside, rest)
            }
            None => (xs, "", ""),
        };
        push_abas(inside, false, &mut inside_abas);
        push_abas(outside, true, &mut outside_abas);
        xs = rest;
    }
    inside_abas.intersection(&outside_abas).next().is_some()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut count = 0;
    for line in BufReader::new(input).lines() {
        count += is_ssl(&line?) as usize;
    }
    writeln!(output, "{count}")?;
    Ok(())
}

adventofcode::main!(solve("examples/07.txt") == "242\n");
