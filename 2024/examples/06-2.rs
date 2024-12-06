use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, NORTH};
use anyhow::Result;

fn walk(
    walls: &HashSet<Coord>, frame: Frame, mut pos: Coord, mut dir: Coord,
) -> Result<(), HashSet<Coord>> {
    let mut visited = HashSet::new();
    while frame.contains(pos) {
        if !visited.insert((pos, dir)) {
            return Ok(());
        }
        let next = pos + dir;
        if walls.contains(&next) {
            dir = dir.right();
            continue;
        }
        pos = next;
    }
    Err(visited.into_iter().map(|(x, _)| x).collect())
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut frame = Frame::default();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        frame.max.y = y;
        for (byte, x) in line?.bytes().zip(0 ..) {
            frame.max.x = x;
            let pos = Coord { x, y };
            match byte {
                b'#' => assert!(walls.insert(pos)),
                b'^' => assert!(start.replace(pos).is_none()),
                b'.' => (),
                _ => panic!("unexpected byte {byte:02x}"),
            }
        }
    }
    let start = start.unwrap();
    let mut total = 0;
    let mut candidates = walk(&walls, frame, start, NORTH).unwrap_err();
    candidates.remove(&start);
    for pos in candidates {
        assert!(walls.insert(pos));
        total += walk(&walls, frame, start, NORTH).is_ok() as usize;
        walls.remove(&pos);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "1888\n");
