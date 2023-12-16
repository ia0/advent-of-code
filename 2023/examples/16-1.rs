use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, EAST, NORTH, SOUTH, WEST};
use anyhow::{bail, ensure, Context, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (cell, x) in line?.bytes().zip(0 ..) {
            let coord = Coord { x, y };
            match cell {
                b'.' => (),
                b'/' | b'\\' | b'-' | b'|' => ensure!(map.insert(coord, cell).is_none()),
                _ => bail!("unexpected cell {cell:?}"),
            }
        }
    }
    let mut frame = Frame::new(map.keys().cloned()).context("empty map")?;
    frame.min = Coord::default();
    let mut visited = HashSet::new();
    let mut todo = vec![(Coord::default(), EAST)];
    while let Some((pos, dir)) = todo.pop() {
        if !frame.contains(pos) || !visited.insert((pos, dir)) {
            continue;
        }
        let dirs = match map.get(&pos) {
            Some(b'|') if [EAST, WEST].contains(&dir) => vec![NORTH, SOUTH],
            Some(b'-') if [NORTH, SOUTH].contains(&dir) => vec![EAST, WEST],
            None | Some(b'|' | b'-') => vec![dir],
            Some(b'/') if dir == EAST => vec![NORTH],
            Some(b'/') if dir == NORTH => vec![EAST],
            Some(b'/') if dir == WEST => vec![SOUTH],
            Some(b'/') if dir == SOUTH => vec![WEST],
            Some(b'\\') if dir == EAST => vec![SOUTH],
            Some(b'\\') if dir == SOUTH => vec![EAST],
            Some(b'\\') if dir == WEST => vec![NORTH],
            Some(b'\\') if dir == NORTH => vec![WEST],
            _ => unreachable!(),
        };
        dirs.into_iter().for_each(|dir| todo.push((pos + dir, dir)));
    }
    let energized = visited.iter().map(|(x, _)| x).collect::<HashSet<_>>();
    writeln!(output, "{}", energized.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/16.txt") == "7242\n");
