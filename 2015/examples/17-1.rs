use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut xs = Vec::new();
    for line in BufReader::new(input).lines() {
        xs.push(line?.parse::<usize>()?);
    }
    let n = xs.len();
    writeln!(
        output,
        "{}",
        (0 ..= 1 << n)
            .filter(|i| (0 .. n).filter_map(|b| (i & 1 << b != 0).then_some(xs[b])).sum::<usize>()
                == 150)
            .count()
    )?;
    Ok(())
}

adventofcode::main!(solve("examples/17.txt") == "1638\n");
