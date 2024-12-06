use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, NORTH};
use anyhow::Result;

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
    let mut pos = start.unwrap();
    let mut dir = NORTH;
    let mut visited = HashSet::new();
    while frame.contains(pos) {
        visited.insert(pos);
        let next = pos + dir;
        if walls.contains(&next) {
            dir = dir.right();
            continue;
        }
        pos = next;
    }
    writeln!(output, "{}", visited.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "5129\n");
