use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, EAST};
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            let pos = Coord { x, y };
            match byte {
                b'#' => assert!(walls.insert(pos)),
                b'S' => assert!(start.replace(pos).is_none()),
                b'E' => assert!(end.replace(pos).is_none()),
                b'.' => (),
                _ => unreachable!(),
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), start, EAST));
    let mut visited = HashSet::new();
    while let Some((Reverse(cost), pos, dir)) = todo.pop() {
        if !visited.insert((pos, dir)) {
            continue;
        }
        if pos == end {
            writeln!(output, "{cost}")?;
            return Ok(());
        }
        if !walls.contains(&(pos + dir)) {
            todo.push((Reverse(cost + 1), pos + dir, dir));
        }
        for dir in [dir.left(), dir.right()] {
            todo.push((Reverse(cost + 1000), pos, dir));
        }
    }
    unreachable!();
}

adventofcode::main!(solve("examples/16.txt") == "66404\n");
