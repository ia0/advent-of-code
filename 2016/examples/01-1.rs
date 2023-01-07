use std::io::{Read, Write};

use adventofcode::Coord;
use anyhow::{bail, Result};

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut directions = String::new();
    input.read_to_string(&mut directions)?;
    let mut pos = Coord::default();
    let mut dir = Coord { x: 0, y: -1 };
    for direction in directions.trim_end().split(", ") {
        let (turn, dist) = direction.split_at(1);
        match turn {
            "R" => dir = Coord { x: dir.y, y: -dir.x },
            "L" => dir = Coord { x: -dir.y, y: dir.x },
            _ => bail!("bad turn {turn}"),
        }
        pos += dir * dist.parse::<i64>()?;
    }
    writeln!(output, "{}", pos.x.abs() + pos.y.abs())?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "209\n");
