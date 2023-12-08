use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Context, Result};

type Tri = [u8; 3];

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let instructions = lines.next().context("no instructions")??.into_bytes();
    ensure!(instructions.iter().all(|x| b"LR".contains(x)));
    ensure!(lines.next().context("no empty line")??.is_empty());
    let mut map = HashMap::new();
    for line in lines {
        let line = line?;
        let (src, dst) = line.split_once(" = ").context("no equal")?;
        let dst = dst.strip_prefix('(').context("no opening parenthesis")?;
        let dst = dst.strip_suffix(')').context("no closing parenthesis")?;
        let (left, right) = dst.split_once(", ").context("no comma")?;
        let src = Tri::try_from(src.as_bytes()).context("src no trigram")?;
        let left = Tri::try_from(left.as_bytes()).context("left no trigram")?;
        let right = Tri::try_from(right.as_bytes()).context("right no trigram")?;
        ensure!(map.insert(src, (left, right)).is_none());
    }
    let mut total = 0;
    let mut cur = *b"AAA";
    let end = *b"ZZZ";
    while cur != end {
        cur = match instructions[total % instructions.len()] {
            b'L' => map[&cur].0,
            b'R' => map[&cur].1,
            _ => unreachable!(),
        };
        total += 1;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "21797\n");
