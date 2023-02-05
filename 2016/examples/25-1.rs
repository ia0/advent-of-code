use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    // The code is equivalent to:
    //
    //   0: d = a + 9 * 282
    //   8: a = d
    //   9: a = a / 2
    //      out a % 2
    //      if a == 0 { goto 8 } else { goto 9 }
    let input = (BufReader::new(input).lines().skip(1).take(2))
        .map(|x| Ok(x?.split_whitespace().collect::<Vec<_>>()[1].parse()?))
        .reduce(|x: Result<usize>, y| Ok(x? * y?))
        .unwrap()?;
    writeln!(output, "{}", (0 ..).find(|x| ((input + x) * 3 / 2 + 1).is_power_of_two()).unwrap())?;
    Ok(())
}

adventofcode::main!(solve("examples/25.txt") == "192\n");
