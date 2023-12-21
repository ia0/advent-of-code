use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, ADJACENT_PLUS};
use anyhow::{bail, ensure, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut frame = Frame { min: Coord::default(), max: Coord::default() };
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        frame.max.y = y;
        for (cell, x) in line?.bytes().zip(0 ..) {
            frame.max.x = x;
            let pos = Coord { x, y };
            match cell {
                b'.' => (),
                b'#' => ensure!(walls.insert(pos)),
                b'S' => ensure!(start.replace(pos).is_none()),
                _ => bail!("invalid cell {cell:?}"),
            }
        }
    }
    ensure!(start.is_some());
    let mut prev = start.into_iter().collect::<HashSet<_>>();
    for _ in 0 .. 64 {
        let mut next = HashSet::new();
        for pos in prev {
            for dir in ADJACENT_PLUS {
                let pos = pos + dir;
                if frame.contains(pos) && !walls.contains(&pos) {
                    next.insert(pos);
                }
            }
        }
        prev = next;
    }
    let total = prev.len();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "3615\n");
