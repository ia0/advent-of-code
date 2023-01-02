use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut xs = Vec::new();
    for line in BufReader::new(input).lines() {
        xs.push(line?.parse::<usize>()?);
    }
    let n = xs.len();
    let mut best = (n as u32, 0);
    for i in 0u32 ..= 1 << n {
        if (0 .. n).filter_map(|b| (i & 1 << b != 0).then_some(xs[b])).sum::<usize>() != 150 {
            continue;
        }
        let k = i.count_ones();
        if best.0 < k {
            continue;
        }
        if best.0 > k {
            best = (k, 0);
        }
        best.1 += 1;
    }
    writeln!(output, "{}", best.1)?;
    Ok(())
}

adventofcode::main!(solve("examples/17.txt") == "17\n");
