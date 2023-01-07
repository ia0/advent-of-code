use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut count = 0;
    for line in BufReader::new(input).lines() {
        let sides =
            line?.split_whitespace().map(|x| Ok(x.parse()?)).collect::<Result<Vec<usize>>>()?;
        assert_eq!(sides.len(), 3);
        if (0 .. 3).all(|i| sides[i] < sides[(i + 1) % 3] + sides[(i + 2) % 3]) {
            count += 1;
        }
    }
    writeln!(output, "{count}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "983\n");
