use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

const DIGITS: &[(&[u8], usize)] = &[
    (b"0", 0),
    (b"1", 1),
    (b"2", 2),
    (b"3", 3),
    (b"4", 4),
    (b"5", 5),
    (b"6", 6),
    (b"7", 7),
    (b"8", 8),
    (b"9", 9),
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];

fn find(line: &[u8], rev: bool) -> usize {
    let iter: Box<dyn Iterator<Item = usize>> = match rev {
        false => Box::new(0 .. line.len()),
        true => Box::new((0 .. line.len()).rev()),
    };
    for i in iter {
        for (pat, val) in DIGITS {
            if line[i ..].starts_with(pat) {
                return *val;
            }
        }
    }
    unreachable!()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?.into_bytes();
        total += find(&line, false) * 10 + find(&line, true);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "54418\n");
