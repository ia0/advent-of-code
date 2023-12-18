use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, ADJACENT_PLUS, EAST, NORTH, SOUTH, WEST};
use anyhow::{bail, ensure, Context, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lagoon = HashSet::new();
    let mut left = HashSet::new();
    let mut pos = Coord::default();
    lagoon.insert(pos);
    for line in BufReader::new(input).lines() {
        let line = line?;
        let words = line.split_whitespace().collect::<Vec<_>>();
        ensure!(words.len() == 3);
        let dir = match words[0] {
            "U" => NORTH,
            "R" => EAST,
            "D" => SOUTH,
            "L" => WEST,
            x => bail!("unexpected direction {x:?}"),
        };
        let len = words[1].parse::<i64>().context("invalid length")?;
        for _ in 0 .. len {
            pos += dir;
            lagoon.insert(pos);
            left.insert(pos + dir.left());
        }
    }
    left.retain(|x| !lagoon.contains(x));
    let mut frame = Frame::new(lagoon.iter().cloned()).context("empty lagoon")?;
    frame.min += Coord { x: -1, y: -1 };
    frame.max += Coord { x: 1, y: 1 };
    let mut todo = left.iter().cloned().collect::<Vec<_>>();
    while let Some(pos) = todo.pop() {
        for dir in ADJACENT_PLUS {
            let pos = pos + dir;
            if !lagoon.contains(&pos) && frame.contains(pos) && left.insert(pos) {
                todo.push(pos);
            }
        }
    }
    if left.contains(&frame.min) {
        writeln!(output, "{}", frame.len() - left.len())?;
    } else {
        writeln!(output, "{}", lagoon.len() + left.len())?;
    }
    Ok(())
}

adventofcode::main!(solve("examples/18.txt") == "66993\n");
