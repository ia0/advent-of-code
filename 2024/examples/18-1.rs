use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    let mut lines = BufReader::new(input).lines();
    for _ in 0 .. N {
        walls.insert(Coord::parse(&lines.next().unwrap()?, ",")?);
    }
    for i in -1 ..= END.x + 1 {
        walls.insert(Coord { x: i, y: -1 });
        walls.insert(Coord { x: i, y: END.x + 1 });
        walls.insert(Coord { x: -1, y: i });
        walls.insert(Coord { x: END.x + 1, y: i });
    }
    let total = adventofcode::shortest_dist_plus(&walls, Coord::default(), END).unwrap();
    writeln!(output, "{total}")?;
    Ok(())
}

const END: Coord = Coord { x: 70, y: 70 };
const N: usize = 1024;
adventofcode::main!(solve("examples/18.txt") == "340\n");
