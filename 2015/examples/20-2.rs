use std::io::{Read, Write};

use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut threshold = String::new();
    input.read_to_string(&mut threshold)?;
    let threshold = threshold.parse::<usize>()?;
    let mut count = vec![11; (threshold + 10) / 11];
    let n = count.len();
    for i in 2 .. n {
        for j in (i .. n).step_by(i).take(50) {
            count[j] += 11 * i;
        }
        if threshold <= count[i] {
            writeln!(output, "{i}")?;
            break;
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/20.txt") == "786240\n");
