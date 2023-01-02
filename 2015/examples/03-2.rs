use std::collections::HashSet;
use std::io::{Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut directions = Vec::new();
    input.read_to_end(&mut directions)?;
    let mut coord = Coord::default();
    let mut other = coord;
    let mut visited = HashSet::new();
    visited.insert(coord);
    for direction in directions {
        match direction {
            b'>' => coord.x += 1,
            b'<' => coord.x -= 1,
            b'v' => coord.y += 1,
            b'^' => coord.y -= 1,
            _ => unreachable!(),
        }
        visited.insert(coord);
        std::mem::swap(&mut coord, &mut other);
    }
    writeln!(output, "{}", visited.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "2360\n");
