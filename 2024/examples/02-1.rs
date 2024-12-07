use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let xs: Vec<_> = adventofcode::parse_ints(&line?)?;
        let mut min = i64::MAX;
        let mut max = i64::MIN;
        for w in xs.windows(2) {
            let d = w[1] - w[0];
            min = std::cmp::min(min, d);
            max = std::cmp::max(max, d);
        }
        total += if max < 0 { -3 <= min && max <= -1 } else { 1 <= min && max <= 3 } as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "246\n");
