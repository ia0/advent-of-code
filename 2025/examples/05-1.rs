use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Intervals;
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut fresh = Intervals::default();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (beg, end) = line.split_once('-').unwrap();
        let beg: i64 = beg.parse()?;
        let end: i64 = end.parse()?;
        fresh.insert(beg .. end + 1);
    }
    let mut total = 0;
    for line in lines {
        let line = line?;
        let id: i64 = line.parse()?;
        total += fresh.contains(id) as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "773\n");
