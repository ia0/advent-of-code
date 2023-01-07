use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let input = (BufReader::new(input).lines())
        .map(|x| x?.split_whitespace().map(|x| Ok(x.parse::<usize>()?)).collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<_>>>()?;
    let mut count = 0;
    for x in input.chunks(3) {
        count += (0 .. 3)
            .filter(|&j| (0 .. 3).all(|i| x[i][j] < x[(i + 1) % 3][j] + x[(i + 2) % 3][j]))
            .count();
    }
    writeln!(output, "{count}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "1836\n");
