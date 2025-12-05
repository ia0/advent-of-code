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
    writeln!(output, "{}", fresh.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "332067203034711\n");
