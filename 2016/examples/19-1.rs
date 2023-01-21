use std::io::{Read, Write};

use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut count = String::new();
    input.read_to_string(&mut count)?;
    let mut count = count.parse::<usize>()?;
    let mut result = 0;
    let mut round = 0;
    while count > 1 {
        round += 1;
        if count % 2 != 0 {
            count -= 1;
            result |= 1 << round;
        }
        count /= 2;
    }
    writeln!(output, "{}", result + 1)?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "1834471\n");
