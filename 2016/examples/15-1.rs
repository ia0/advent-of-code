use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use regex::Regex;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let regex =
        Regex::new(r"^Disc #(.*) has (.*) positions; at time=0, it is at position (.*)\.$")?;
    let mut discs = Vec::<(i64, i64)>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let captures = regex.captures(&line).unwrap();
        let i = discs.len() as i64 + 1;
        discs.push((-i - captures[3].parse::<i64>()?, captures[2].parse()?));
        assert_eq!(captures[1].parse::<usize>()?, discs.len());
    }
    writeln!(output, "{}", adventofcode::crt(&discs).0)?;
    Ok(())
}

adventofcode::main!(solve("examples/15.txt") == "16824\n");
