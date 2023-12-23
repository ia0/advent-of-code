use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, ADJACENT_PLUS, EAST, NORTH, SOUTH, WEST};
use anyhow::{ensure, Context, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (cell, x) in line?.bytes().zip(0 ..) {
            let coord = Coord { x, y };
            ensure!(b"#.<>^v".contains(&cell));
            map.insert(coord, cell);
        }
    }
    let frame = Frame::new(map.keys().cloned()).context("empty map")?;
    let start = Coord { x: frame.min.x + 1, y: frame.min.y };
    let end = Coord { x: frame.max.x - 1, y: frame.max.y };
    let mut best = 0;
    let mut todo = vec![(HashSet::<Coord>::new(), 0, start)];
    while let Some((mut visited, dist, pos)) = todo.pop() {
        let mut next = Vec::new();
        for dir in ADJACENT_PLUS {
            let mut pos = pos + dir;
            let mut dist = dist + 2;
            match map.get(&pos) {
                Some(b'.') => dist -= 1,
                Some(b'^') => pos += NORTH,
                Some(b'v') => pos += SOUTH,
                Some(b'<') => pos += WEST,
                Some(b'>') => pos += EAST,
                Some(b'#') | None => continue,
                _ => unreachable!(),
            }
            if pos == end {
                best = std::cmp::max(best, dist);
            } else if !visited.contains(&pos) {
                next.push((dist, pos));
            }
        }
        let mut next = next.into_iter();
        let last = match next.next() {
            Some(x) => x,
            None => continue,
        };
        for (dist, pos) in next {
            let mut visited = visited.clone();
            assert!(visited.insert(pos));
            todo.push((visited, dist, pos));
        }
        let (dist, pos) = last;
        assert!(visited.insert(pos));
        todo.push((visited, dist, pos));
    }
    writeln!(output, "{best}")?;
    Ok(())
}

adventofcode::main!(solve("examples/23.txt") == "2110\n");
