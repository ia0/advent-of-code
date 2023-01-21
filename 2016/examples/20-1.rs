use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Intervals;
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut banned = Intervals::default();
    for line in BufReader::new(input).lines() {
        let words = line?.split('-').map(|x| Ok(x.parse::<i64>()?)).collect::<Result<Vec<_>>>()?;
        assert_eq!(words.len(), 2);
        banned.insert(words[0] .. words[1] + 1);
    }
    let mut min = 0;
    for ban in &banned[..] {
        if min < ban.start {
            break;
        }
        min = ban.end;
    }
    writeln!(output, "{min}")?;
    Ok(())
}

adventofcode::main!(solve("examples/20.txt") == "19449262\n");
