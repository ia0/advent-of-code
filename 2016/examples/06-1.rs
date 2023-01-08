use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let data = BufReader::new(input).lines().map(|x| Ok(x?)).collect::<Result<Vec<_>>>()?;
    let mut count = vec![HashMap::<char, usize>::new(); data[0].len()];
    assert!(data.iter().all(|x| x.len() == count.len()));
    for data in data {
        for (i, c) in data.chars().enumerate() {
            *count[i].entry(c).or_default() += 1;
        }
    }
    let mut message = String::new();
    for count in count {
        message.push(count.into_iter().map(|(c, n)| (n, c)).max().unwrap().1);
    }
    writeln!(output, "{message}")?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "gebzfnbt\n");
