use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let dimensions: Vec<usize> =
            line?.split('x').map(|x| x.parse()).collect::<Result<_, _>>()?;
        assert_eq!(dimensions.len(), 3);
        let mut sides = [0; 3];
        for i in 0 .. 3 {
            sides[i] = dimensions[i] * dimensions[(i + 1) % 3];
        }
        total += 2 * sides.iter().sum::<usize>();
        total += sides.iter().min().unwrap();
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "1586300\n");
