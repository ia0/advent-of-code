use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn is_nice(xs: &[u8]) -> bool {
    let mut pos = HashMap::new();
    for (i, xy) in xs.windows(2).enumerate() {
        match pos.entry(xy) {
            Entry::Occupied(j) => {
                if *j.get() < i - 1 {
                    return xs.windows(3).any(|x| x[0] == x[2]);
                }
            }
            #[allow(clippy::drop_ref)]
            Entry::Vacant(x) => drop(x.insert(i)),
        }
    }
    false
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?.into_bytes();
        total += is_nice(&line) as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "55\n");
