use std::io::{Read, Write};

use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut threshold = String::new();
    input.read_to_string(&mut threshold)?;
    let threshold = threshold.parse::<usize>()?;
    let mut count = vec![10; threshold.div_ceil(10)];
    let n = count.len();
    for i in 2 .. n {
        for j in (i .. n).step_by(i) {
            count[j] += 10 * i;
        }
        if threshold <= count[i] {
            writeln!(output, "{i}")?;
            break;
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/20.txt") == "776160\n");
