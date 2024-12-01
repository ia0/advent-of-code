use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut left = Vec::<i64>::new();
    let mut right = HashMap::<i64, i64>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        assert_eq!(words.len(), 2);
        left.push(words[0].parse()?);
        *right.entry(words[1].parse()?).or_default() += 1;
    }
    let mut total = 0;
    for x in left {
        total += x * right.get(&x).unwrap_or(&0);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "24869388\n");
