use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn area(a: Coord, b: Coord) -> usize {
    (((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)) as usize
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut red = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (x, y) = line.split_once(',').unwrap();
        red.push(Coord { x: x.parse()?, y: y.parse()? });
    }
    let mut max = 0;
    for (i, &a) in red.iter().enumerate() {
        for &b in red.iter().take(i) {
            max = std::cmp::max(max, area(a, b));
        }
    }
    writeln!(output, "{max}")?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "4759531084\n");
