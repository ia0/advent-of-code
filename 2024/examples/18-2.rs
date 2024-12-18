use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    for i in -1 ..= END.x + 1 {
        walls.insert(Coord { x: i, y: -1 });
        walls.insert(Coord { x: i, y: END.x + 1 });
        walls.insert(Coord { x: -1, y: i });
        walls.insert(Coord { x: END.x + 1, y: i });
    }
    let good = walls.len() + 1024;
    for line in BufReader::new(input).lines() {
        let byte = line?;
        walls.insert(Coord::parse(&byte, ",")?);
        if walls.len() < good {
            continue;
        }
        if adventofcode::shortest_dist_plus(&walls, Coord::default(), END).is_none() {
            writeln!(output, "{byte}")?;
            break;
        }
    }
    Ok(())
}

const END: Coord = Coord { x: 70, y: 70 };
adventofcode::main!(solve("examples/18.txt") == "34,32\n");
