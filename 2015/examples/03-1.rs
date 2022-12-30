use std::collections::HashSet;
use std::io::{Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut directions = Vec::new();
    input.read_to_end(&mut directions)?;
    let mut coord = Coord::default();
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
    }
    writeln!(output, "{}", visited.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "2592\n");
