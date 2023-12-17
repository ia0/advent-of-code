use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, ADJACENT_PLUS, EAST, SOUTH};
use anyhow::{ensure, Context, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (cell, x) in line?.bytes().zip(0 ..) {
            let coord = Coord { x, y };
            ensure!(cell.is_ascii_digit());
            ensure!(map.insert(coord, (cell - b'0') as i64).is_none());
        }
    }
    let frame = Frame::new(map.keys().cloned()).context("empty map")?;
    let start = frame.min;
    let end = frame.max;
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), start, EAST, 0));
    todo.push((Reverse(0), start, SOUTH, 0));
    let mut visited = HashSet::new();
    while let Some((Reverse(heat), pos, dir, len)) = todo.pop() {
        if !visited.insert((pos, dir, len)) {
            continue;
        }
        if pos == end {
            writeln!(output, "{heat}")?;
            return Ok(());
        }
        for next in ADJACENT_PLUS {
            if next == -dir || (next == dir && len == 3) {
                continue;
            }
            let pos = pos + next;
            if let Some(diff) = map.get(&pos) {
                todo.push((Reverse(heat + diff), pos, next, (next == dir) as i64 * len + 1));
            }
        }
    }
    unreachable!()
}

adventofcode::main!(solve("examples/17.txt") == "814\n");
