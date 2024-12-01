use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        assert_eq!(words.len(), 2);
        left.push(words[0].parse()?);
        right.push(words[1].parse()?);
    }
    left.sort();
    right.sort();
    let total: i64 = left.iter().zip(right.iter()).map(|(x, y)| (x - y).abs()).sum();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "1110981\n");
