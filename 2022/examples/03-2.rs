use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn priority(x: u8) -> usize {
    (match x {
        b'a' ..= b'z' => x - b'a' + 1,
        b'A' ..= b'Z' => x - b'A' + 27,
        _ => unreachable!(),
    }) as usize
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    let mut lines = BufReader::new(input).lines().map(|x| x.unwrap().into_bytes());
    while let Some(line1) = lines.next() {
        let group = [line1, lines.next().unwrap(), lines.next().unwrap()];
        let inter = group
            .iter()
            .map(|x| x.iter().cloned().collect::<HashSet<u8>>())
            .reduce(|x, y| x.intersection(&y).cloned().collect())
            .unwrap();
        assert_eq!(inter.len(), 1);
        total += priority(inter.into_iter().next().unwrap());
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "2497\n");
